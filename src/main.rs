mod app_state;
mod rk8solver;
mod runner;
mod render_export;

use std::{collections::HashSet, thread, time::Duration}; // tmp
use app_state::{AppState, Exports};
use eframe::egui;

impl eframe::App for AppState {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("My egui Application");

            ui.add(egui::Slider::new(&mut self.pendulum_params.gravity, 0.0..=10.0).text("value"));

            if ui.button("Start").clicked() {
                self.start_simulation();
            }
            if ui.button("Stop").clicked() {
                self.stop_simulation();
            }
        });
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native("My egui App", native_options, Box::new(|cc| Ok(Box::new(AppState::default()))));

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
