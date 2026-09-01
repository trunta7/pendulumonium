mod app_state;
mod selector;
mod rk8solver;
mod runner;
mod render_export;

use app_state::{AppState, Exports, ActivePanel};
use eframe::egui::{self, Pos2, Rect};

use crate::{render_export::RenderExportConfig, rk8solver::PendulumParams, runner::RunnerConfig, selector::PointSelector};

const SEL_WIDTH: f32 = 200.0;

impl eframe::App for AppState {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // left application control panel, handles toggling exports and start/stop simulation
        self.display_application_control(ui);

        // top panel selector panel, handles choosing the current settings to display
        self.display_panel_selector(ui);

        // central active panel
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

            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut self.selector.active, selector::ActiveSelector::RectSel, "Rectangle Selection"
                );
                ui.selectable_value(
                    &mut self.selector.active, selector::ActiveSelector::PointSel, "Point Selection"
                );
            });

            let desired_size = egui::vec2(SEL_WIDTH, SEL_WIDTH);
            let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
            ui.painter().rect_filled(rect, 1.0, egui::Color32::from_rgb(60, 60, 60));

            if response.clicked() {
                if let Some(pointer_pos) = response.interact_pointer_pos() {
                    let relative_pos = pointer_pos - rect.min;
                    println!("Relative click position: {:?}", relative_pos);
                    let scaled_pos = (relative_pos / SEL_WIDTH).to_pos2();
                    self.selector.add_selection(scaled_pos);
                }
            }

            match self.selector.active {
                selector::ActiveSelector::RectSel => self.display_rectangle_selection(ui, rect),
                selector::ActiveSelector::PointSel => self.display_point_selection(ui, rect),
            }
        });
    }

    fn display_rectangle_selection(&mut self, ui: &mut egui::Ui, rect: Rect) {
        let min = scaled_pos_to_absolute_pos(self.selector.get_points()[0], rect);
        let max = scaled_pos_to_absolute_pos(self.selector.get_points()[1], rect);
        let absolute_box = egui::Rect::from_min_max(min, max,);

        let clipped_painter = ui.painter_at(rect);
        clipped_painter.rect(
            absolute_box,
            2.0, // Corner rounding radius
            egui::Color32::from_rgba_unmultiplied(0, 120, 255, 60), // Translucent blue (R, G, B, A)
            egui::Stroke::new(2.0, egui::Color32::from_rgb(0, 180, 255)), // Solid border stroke
            egui::StrokeKind::Inside
        );

        ui.add(
            egui::Slider::new(&mut self.selector.rect.width, (2..=2000)).step_by(2.0).text("Width")
        ).on_hover_text("Number of pendulums to render on the width of the selection");

        if ui.button("Reset to default").clicked() {
            self.selector.rect = selector::RectSelector::default();
        }
    }

    fn display_point_selection(&mut self, ui: &mut egui::Ui, rect: Rect) {
        for point in self.selector.get_points() {
            let pos = scaled_pos_to_absolute_pos(point, rect);
                ui.painter().circle_filled(
                pos,
                1.0, // Radius in points
                egui::Color32::LIGHT_BLUE,
            );
        }

        if ui.button("Reset to default").clicked() {
            self.selector.point = PointSelector::default();
        }
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

fn scaled_pos_to_absolute_pos(scaled_pos: Pos2, rect: Rect) -> Pos2 {
    (scaled_pos * SEL_WIDTH) + rect.min.to_vec2()
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native("My egui App", native_options, Box::new(|_cc| Ok(Box::new(AppState::default()))));
}
