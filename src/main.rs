mod rk8solver;
mod runner;
mod render_export;

use rtrb::RingBuffer;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;




fn main() {
    let mut init_pends: Vec<rk8solver::State> = Vec::new();
    for i in 1..20 {
        println!("hi");
        init_pends.push(rk8solver::State { 
            theta1: i as f64 * 0.1, 
            theta2: i as f64 * 0.1, 
            omega1: 0.0, 
            omega2: 0.0, 
        });
    }

    let pend_params = rk8solver::PendulumParams {
        mass1: 1.0,
        mass2: 1.0,
        length1: 1.0,
        length2: 1.0,
        gravity: 2.0
    };
    let config = render_export::RenderExportConfig{
        n: 20,
    };
    let stop_flag = Arc::new(AtomicBool::new(false));
    let mut producers = Vec::new();
    let (producer, consumer) = RingBuffer::<Arc<Vec<rk8solver::State>>>::new(60);
    producers.push(producer);
    thread::spawn(move || {render_export::render_export(consumer, &pend_params, config, stop_flag);});

    let config = runner::RunnerConfig::default();
    thread::spawn(move || {runner::run_export(producers, init_pends, pend_params, config);});
    loop {

    }
}
