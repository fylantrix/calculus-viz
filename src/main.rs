use eframe::egui;
use std::f64::consts::PI;
// use egui_plot::{Legend, Line, Plot, PlotPoints};
mod derivatives;
use crate::derivatives::*;
mod integrals;
use crate::integrals::*;

const LOWER: f64 = -2.0 * PI;
const UPPER: f64 = 2.0 * PI;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    Derivative,
    Integral,
}

struct VisualizerApp {
    mode: Mode,
    function_chart: DerivativeChart,
    area_chart: IntegralChart,
}

impl Default for VisualizerApp {
    fn default() -> Self {
        Self {
            mode: Mode::Derivative,
            function_chart: DerivativeChart::default(),
            area_chart: IntegralChart::default(),
        }
    }
}

impl VisualizerApp {
}

impl eframe::App for VisualizerApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.mode, Mode::Derivative, "Derivative");
            ui.selectable_value(&mut self.mode, Mode::Integral, "Integral");
        });

        ui.separator();

        match self.mode {
            Mode::Derivative => self.function_chart.show(ui),
            Mode::Integral => self.area_chart.show(ui),
        }
    }
}


fn main() -> eframe::Result {
    eframe::run_native(
        "Function Visualizer",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(VisualizerApp::default()))),
    )
}
