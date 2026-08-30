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
        self.display_application_control(ui);

        // top panel selector panel, handles choosing the current settings to display
        self.display_panel_selector(ui);

        match self.active_panel {
            ActivePanel::SimulationPanel => {
                self.display_simulation_panel(ui);
            }
            ActivePanel::SelectionPanel => {
                self.display_selection_panel(ui);
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

    fn display_application_control(&mut self, ui: &mut egui::Ui) {
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
    }

    fn display_panel_selector(&mut self, ui: &mut egui::Ui) {
        egui::Panel::top("panel_selector")
        .default_size(50.0)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut self.active_panel, ActivePanel::SimulationPanel, "Simulation"
                );
                ui.selectable_value(
                    &mut self.active_panel, ActivePanel::SelectionPanel, "Selection"
                );
                if self.exports[Exports::RenderExport] {
                    ui.selectable_value(
                    &mut self.active_panel, ActivePanel::RenderExportPanel, "Render Export"
                );
                }
            })
        });
    }

    fn display_simulation_panel(&mut self, ui: &mut egui::Ui) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Pendulum Parameters");
            ui.add(
                egui::Slider::new(&mut self.pendulum_params.mass1, 0.0..=10.0).text("Mass 1")
            ).on_hover_text("Adjusts the mass of the first pendulum");
            ui.add(
                egui::Slider::new(&mut self.pendulum_params.mass2, 0.0..=10.0).text("Mass 2")
            ).on_hover_text("Adjusts the mass of the second pendulum");
            ui.add(
                egui::Slider::new(&mut self.pendulum_params.length1, 0.0..=10.0).text("Length 1")
            ).on_hover_text("Adjusts the length of the first pendulum");
            ui.add(
                egui::Slider::new(&mut self.pendulum_params.length2, 0.0..=10.0).text("Length 2")
            ).on_hover_text("Adjusts the length of the seconds pendulum");
            ui.add(
                egui::Slider::new(&mut self.pendulum_params.gravity, -10.0..=10.0).text("Gravity")
            ).on_hover_text("Adjusts the gravity of the simulation");
            if ui.button("Reset to default").clicked() {
                self.pendulum_params = PendulumParams::default();
            }
            
            ui.separator();
            ui.heading("Simulation Settings");
            ui.add(
                egui::Slider::new(&mut self.runner_config.rtol, 1e-9..=0.01).text("Relative Tolerance")
                .custom_formatter(|val, _range| format!("{:.2e}", val))
            ).on_hover_text("Adjusts the relative error tolerance allowed for the variable step");
            ui.add(
                egui::Slider::new(&mut self.runner_config.atol, 1e-9..=0.01).text("Absolute Tolerance")
                .custom_formatter(|val, _range| format!("{:.2e}", val)) 

            ).on_hover_text("Adjusts the absolute error tolerance allowed for the variable step");
            ui.add(
                egui::Slider::new(&mut self.runner_config.initial_dt, 1e-9..=1.0).text("Initial DT")
                .custom_formatter(|val, _range| format!("{:.2e}", val)) 
            ).on_hover_text("Adjusts the initial DT for the solver to try");
            ui.add(
                egui::Slider::new(&mut self.runner_config.min_dt, 1e-9..=0.001).text("Min DT")
                .custom_formatter(|val, _range| format!("{:.2e}", val)) 
            ).on_hover_text("Adjusts the minimum DT that the solver can go down to");
            ui.add(
                egui::Slider::new(&mut self.runner_config.max_dt, 1e-9..=0.1).text("Max DT")
                .custom_formatter(|val, _range| format!("{:.2e}", val)) 
            ).on_hover_text("Adjusts the maximum DT that the solver can go up to");
            ui.add(
                egui::Slider::new(&mut self.runner_config.frame_time, 1e-6..=1.0).text("Frame Time")
            ).on_hover_text("Adjusts the time difference between each export frame");
            if ui.button("Reset to default").clicked() {
                self.runner_config = RunnerConfig::default();
            }
            
        });
    }

    fn display_selection_panel(&mut self, ui: &mut egui::Ui) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Selection :)");
        });
    }

    fn display_render_export_panel(&mut self, ui: &mut egui::Ui) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Render Export Settings");
            ui.add(
                egui::Slider::new(&mut self.render_export_config.n, 1..=100).text("Render Number")
            ).on_hover_text("Number of pendulums to render from the selection");
            ui.add(
                egui::Slider::new(&mut self.render_export_config.window_width, 10..=2000).text("Window Width")
            ).on_hover_text("Width of the render window in pixels");
            ui.add(
                egui::Slider::new(&mut self.render_export_config.window_height, 10..=2000).text("Window Height")
            ).on_hover_text("Height of the render window in pixels");
            ui.add(
                egui::Slider::new(&mut self.render_export_config.pixel_scale, 0.0..=300.0).text("Pixel Scale")
            ).on_hover_text("Pixels per unit of length");
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
