use crate::rk8solver::{evaluate_derivatives, PendulumParams, State};
use rayon::prelude::*;
use rtrb::Producer;
use std::sync::Arc;

pub struct RunnerConfig {
    pub rtol: f64,
    pub atol: f64,
    pub initial_dt: f64,
    pub min_dt: f64,
    pub max_dt: f64,
    pub frame_time: f64, // e.g., 1.0 / 60.0
}

impl Default for RunnerConfig {
    fn default() -> Self {
        Self {
            rtol: 1e-7,
            atol: 1e-7,
            initial_dt: 0.01,
            min_dt: 1e-9,
            max_dt: 0.1,
            frame_time: 1.0/60.0,
        }
    }
}

/// Cubic Hermite Interpolation for a single pendulum.
#[inline]
pub fn interpolate(
    y0: &State,
    y1: &State,
    f0: &State,
    f1: &State,
    dt: f64,
    theta: f64,
) -> State {
    let t2 = theta * theta;
    let t3 = t2 * theta;

    let h00 = 2.0 * t3 - 3.0 * t2 + 1.0;
    let h10 = t3 - 2.0 * t2 + theta;
    let h01 = -2.0 * t3 + 3.0 * t2;
    let h11 = t3 - t2;

    let mut interp = State::default();
    interp = interp.add(&y0.mul_scalar(h00));
    interp = interp.add(&y1.mul_scalar(h01));
    interp = interp.add(&f0.mul_scalar(dt * h10));
    interp = interp.add(&f1.mul_scalar(dt * h11));

    interp
}

pub fn interpolate_swarm(
    y0s: &[State],
    y1s: &[State],
    f0s: &[State],
    f1s: &[State],
    dt: f64,
    theta: f64,
) -> Vec<State> {
    y0s.par_iter()
        .zip(y1s.par_iter())
        .zip(f0s.par_iter().zip(f1s.par_iter()))
        .map(|((y0, y1), (f0, f1))| interpolate(y0, y1, f0, f1, dt, theta))
        .collect()
}

/// main execution loop supporting a swarm of pendulums and multiple consumers via Arc fan-out.
pub fn run_export(
    mut producers: Vec<Producer<Arc<Vec<State>>>>,
    initial_states: Vec<State>,
    params: PendulumParams,
    config: RunnerConfig,
) {
    // DOP853 Error Estimator Constants
    const ER1: f64 = 0.1312004499419488073250102996e-01;
    const ER6: f64 = -0.1225156446376204440720569753e+01;
    const ER7: f64 = -0.4957589496572501915214079952;
    const ER8: f64 = 0.1664377182454986536961530415e+01;
    const ER9: f64 = -0.3503288487499736816886487290;
    const ER10: f64 = 0.3341791187130174790297318841;
    const ER11: f64 = 0.8192320648511571246570742613e-01;
    const ER12: f64 = -0.2235530786388629525884427845e-01;

    let mut current_t = 0.0;
    let mut next_frame_time = 0.0;
    let mut dt = config.initial_dt;

    let mut old_states = initial_states.clone();
    let mut current_states = initial_states;

    let mut f_old: Vec<State> = old_states
        .par_iter()
        .map(|s| evaluate_derivatives(s, &params))
        .collect();
    let mut f_current = f_old.clone();

    loop {
        let mut dt_taken = dt;
        let mut step_accepted = false;

        while !step_accepted {
            // Parallel advance using Rayon across the entire swarm
            let swarm_results: Vec<((State, [State; 12]), State)> = current_states
                .par_iter()
                .map(|s| {
                    let res = crate::rk8solver::rk8_step(s, &params, dt);
                    let f_new = evaluate_derivatives(&res.0, &params);
                    (res, f_new)
                })
                .collect();

            let mut max_swarm_err = 1e-10_f64;

            for (i, (res, _)) in swarm_results.iter().enumerate() {
                let candidate_state = res.0;
                let k_vals = &res.1;

                let mut err_state = State::default();
                err_state = err_state.add(&k_vals[0].mul_scalar(dt * ER1));
                err_state = err_state.add(&k_vals[5].mul_scalar(dt * ER6));
                err_state = err_state.add(&k_vals[6].mul_scalar(dt * ER7));
                err_state = err_state.add(&k_vals[7].mul_scalar(dt * ER8));
                err_state = err_state.add(&k_vals[8].mul_scalar(dt * ER9));
                err_state = err_state.add(&k_vals[9].mul_scalar(dt * ER10));
                err_state = err_state.add(&k_vals[10].mul_scalar(dt * ER11));
                err_state = err_state.add(&k_vals[11].mul_scalar(dt * ER12));

                let curr = current_states[i];
                let scale_t1 = config.atol + config.rtol * curr.theta1.abs().max(candidate_state.theta1.abs());
                max_swarm_err = max_swarm_err.max((err_state.theta1 / scale_t1).abs());

                let scale_t2 = config.atol + config.rtol * curr.theta2.abs().max(candidate_state.theta2.abs());
                max_swarm_err = max_swarm_err.max((err_state.theta2 / scale_t2).abs());

                let scale_o1 = config.atol + config.rtol * curr.omega1.abs().max(candidate_state.omega1.abs());
                max_swarm_err = max_swarm_err.max((err_state.omega1 / scale_o1).abs());

                let scale_o2 = config.atol + config.rtol * curr.omega2.abs().max(candidate_state.omega2.abs());
                max_swarm_err = max_swarm_err.max((err_state.omega2 / scale_o2).abs());
            }

            let mut dt_factor = 0.9 * (1.0 / max_swarm_err).powf(1.0 / 8.0);
            dt_factor = dt_factor.clamp(0.1, 5.0);

            if max_swarm_err <= 1.0 {
                step_accepted = true;
                dt_taken = dt;

                old_states = current_states;
                current_states = swarm_results.iter().map(|(res, _)| res.0).collect();
                f_old = f_current;
                f_current = swarm_results.iter().map(|(_, f_new)| *f_new).collect();

                current_t += dt_taken;
                dt = (dt * dt_factor).clamp(config.min_dt, config.max_dt);
            } else {
                dt *= dt_factor;
                if dt < config.min_dt {
                    panic!("solver diverged, target dt fell below min_dt threshold.");
                }
            }
        }

        // output frames that fall within the accepted step window
        while next_frame_time <= current_t {
            let step_start_time = current_t - dt_taken;

            let frame_states = if (next_frame_time - current_t).abs() < 1e-9 {
                current_states.clone()
            } else {
                let theta = (next_frame_time - step_start_time) / dt_taken;
                interpolate_swarm(&old_states, &current_states, &f_old, &f_current, dt_taken, theta)
            };

            // wrap frame in an Arc for zero-copy fan-out to multiple consumers
            let shared_frame = Arc::new(frame_states);

            producers.retain_mut(|producer| {
                while producer.is_full() {
                    if producer.is_abandoned() {
                        return false; // Remove abandoned consumer
                    }
                    std::thread::yield_now();
                }

                if producer.is_abandoned() {
                    return false;
                }

                let _ = producer.push(Arc::clone(&shared_frame));
                true
            });

            // stop simulation if all consumers have disconnected
            if producers.is_empty() {
                return;
            }

            next_frame_time += config.frame_time;
        }
    }
}