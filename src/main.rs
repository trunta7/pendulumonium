mod app_state;
mod rk8solver;
mod runner;
mod render_export;

use std::{collections::HashSet, thread, time::Duration}; // tmp
use app_state::{AppState, Exports, ActivePanel};
use eframe::{App, egui};

use crate::{render_export::RenderExportConfig, rk8solver::PendulumParams, runner::RunnerConfig};

impl eframe::App for AppState {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // left application control panel, handles toggling exports and start/stop simulation
        egui::Panel::left("application_control")
        .default_size(200.0)
        .show(ui, |ui| {
            ui.heading("Exports");
            ui.checkbox(&mut self.exports[Exports::RenderExport], "Render Export");
            ui.separator();

            ui.heading("Simulation");
            if ui.button("Start").clicked() {
                self.start_simulation();
            }
            if ui.button("Stop").clicked() {
                self.stop_simulation();
            }
        });

        // top settings selector panel, handles choosing the current settings to display
        egui::Panel::top("settings_selector")
        .default_size(50.0)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut self.active_panel, ActivePanel::SimulationPanel, "Simulation"
                );
                if self.exports[Exports::RenderExport] {
                    ui.selectable_value(
                    &mut self.active_panel, ActivePanel::RenderExportPanel, "Render Export"
                );
                }
            })
        });

        match self.active_panel {
            ActivePanel::SimulationPanel => {
                self.display_simulation_panel(ui);
            }
            ActivePanel::RenderExportPanel => {
                if !self.exports[Exports::RenderExport] {
                    self.active_panel = ActivePanel::default();
                } else {
                    self.display_render_export_panel(ui);
                }
            }
        }
    }
}

impl AppState {

    fn display_simulation_panel(&mut self, ui: &mut egui::Ui) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Pendulum Parameters");
            ui.add(
                egui::Slider::new(&mut self.pendulum_params.mass1, 0.0..=10.0).text("Mass 1")
            );
            ui.add(
                egui::Slider::new(&mut self.pendulum_params.mass2, 0.0..=10.0).text("Mass 2")
            );
            ui.add(
                egui::Slider::new(&mut self.pendulum_params.length1, 0.0..=10.0).text("Length 1")
            );
            ui.add(
                egui::Slider::new(&mut self.pendulum_params.length2, 0.0..=10.0).text("Length 2")
            );
            ui.add(
                egui::Slider::new(&mut self.pendulum_params.gravity, -10.0..=10.0).text("Gravity")
            );
            if ui.button("Reset to default").clicked() {
                self.pendulum_params = PendulumParams::default();
            }
            
            ui.separator();
            ui.heading("Simulation Settings");
            ui.add(
                egui::Slider::new(&mut self.runner_config.rtol, 1e-9..=0.01).text("Relative Tolerance")
                .custom_formatter(|val, _range| format!("{:.2e}", val)) 
            );
            ui.add(
                egui::Slider::new(&mut self.runner_config.atol, 1e-9..=0.01).text("Absolute Tolerance")
                .custom_formatter(|val, _range| format!("{:.2e}", val)) 
            );
            ui.add(
                egui::Slider::new(&mut self.runner_config.initial_dt, 1e-9..=1.0).text("Initial DT")
                .custom_formatter(|val, _range| format!("{:.2e}", val)) 
            );
            ui.add(
                egui::Slider::new(&mut self.runner_config.min_dt, 1e-9..=0.001).text("Min DT")
                .custom_formatter(|val, _range| format!("{:.2e}", val)) 
            );
            ui.add(
                egui::Slider::new(&mut self.runner_config.max_dt, 1e-9..=0.1).text("Max DT")
                .custom_formatter(|val, _range| format!("{:.2e}", val)) 
            );
            ui.add(
                egui::Slider::new(&mut self.runner_config.frame_time, 1e-6..=1.0).text("Frame Time")
            );
            if ui.button("Reset to default").clicked() {
                self.runner_config = RunnerConfig::default();
            }
            
        });
    }

    fn display_render_export_panel(&mut self, ui: &mut egui::Ui) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Render Export Settings");
            ui.add(
                egui::Slider::new(&mut self.render_export_config.n, 1..=100).text("Render Number")
            );
            ui.add(
                egui::Slider::new(&mut self.render_export_config.window_width, 10..=2000).text("Window Width")
            );
            ui.add(
                egui::Slider::new(&mut self.render_export_config.window_height, 10..=2000).text("Window Height")
            );
            ui.add(
                egui::Slider::new(&mut self.render_export_config.pixel_scale, 0.0..=300.0).text("Pixel Scale")
            );
            if ui.button("Reset to default").clicked() {
                self.render_export_config = RenderExportConfig::default();
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
