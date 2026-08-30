use rtrb::{Consumer, RingBuffer};

use crate::app_state::Exports::RenderExport;
use crate::rk8solver::{State, PendulumParams};
use crate::runner::{self, RunnerConfig};
use crate::render_export;

use std::collections::HashSet;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

const RING_SIZE: usize = 60;

#[derive(PartialEq, Eq, Hash)]
pub enum Exports {
    RenderExport,
}
pub struct AppState {
	pub initial_pendulums: Vec<State>, // vector of initial pends to simulate
    pub pendulum_params: PendulumParams, // params of pendulums
	pub runner_config: RunnerConfig, // config of runner
    pub exports: HashSet<Exports>, // vec of selected exports
	pub render_export_config: render_export::RenderExportConfig, // config of the render export
	pub stop_flag: Arc<AtomicBool>, // stop flag to stop export threads
}

impl Default for AppState {
    fn default() -> Self {
        Self { 
            initial_pendulums: vec![State{theta1:1.0, theta2:1.0, omega1:0.0, omega2:0.0}],
            pendulum_params: PendulumParams::default(),
            runner_config: RunnerConfig::default(), 
            exports: HashSet::from([RenderExport]),
            render_export_config: render_export::RenderExportConfig::default(), 
            stop_flag: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl AppState {
    pub fn start_simulation(&self) {
        self.stop_flag.store(false, Ordering::Relaxed);

        let mut producers = Vec::new();
        for exp in &self.exports {
            let (producer, consumer) = RingBuffer::<Arc<Vec<State>>>::new(RING_SIZE);
            producers.push(producer);
            match exp {
                RenderExport => {
                    self.spawn_render_export(consumer);
                }
            }
        }

        let initial_pends = self.initial_pendulums.clone();
        let pend_params = self.pendulum_params.clone();
        let runner_config = self.runner_config.clone();
        thread::spawn(move || {
            runner::run_export(
                producers, 
                initial_pends, 
                pend_params, 
                runner_config
            );
        });
    }

    pub fn stop_simulation(&self) {
        self.stop_flag.store(true, Ordering::Relaxed);
    }

    fn spawn_render_export(&self, consumer: Consumer<Arc<Vec<State>>>) -> std::thread::JoinHandle<()> {
        let pendulum_params = self.pendulum_params.clone();
        let render_config = self.render_export_config.clone();
        let stop_flag = self.stop_flag.clone();

        thread::spawn(move || {
            render_export::render_export(
                consumer,
                pendulum_params,
                render_config,
                stop_flag,
            );
        })
    }
}