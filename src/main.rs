mod app_state;
mod rk8solver;
mod runner;
mod render_export;

use std::{collections::HashSet, thread, time::Duration}; // tmp
use app_state::{AppState, Exports};


fn main() {
    

    /*
    let mut init_pends: Vec<rk8solver::State> = Vec::new();
    for i in 0..20 {
        init_pends.push(rk8solver::State { 
            omega1: i as f64 * 0.1, 
            omega2: i as f64 * 0.1, 
            theta1: 0.0, 
            theta2: 0.0, 
        });
    }

    let mut state = AppState::default();
    state.initial_pendulums = init_pends;
    state.exports = HashSet::from([Exports::RenderExport]);
    state.pendulum_params.gravity = 2.0;
    state.start_simulation();
    thread::sleep(Duration::from_secs(10));
    state.stop_simulation();
    thread::sleep(Duration::from_secs(2));
    */
}
