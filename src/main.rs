use eframe::egui;
mod derivatives;
use crate::derivatives::*;
mod integrals;
use crate::integrals::*;
mod taylor;
use crate::taylor::*;
use calculus::{LOWER, UPPER, Function};


#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    Derivative,
    Integral,
}

struct VisualizerApp {
    mode: Mode,
    function: Function,
    function_chart: DerivativeChart,
    area_chart: IntegralChart,
}

impl Default for VisualizerApp {
    fn default() -> Self {
        Self {
            mode: Mode::Derivative,
            function: Function::Sin,
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

        egui::ComboBox::from_label("Function")
            .selected_text(self.function.name())
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.function, Function::Sin, "sin(x)");
                ui.selectable_value(&mut self.function, Function::Cos, "cos(x)");
                ui.selectable_value(&mut self.function, Function::Square, "x²");
                ui.selectable_value(&mut self.function, Function::Cube, "x³");
                ui.selectable_value(
                    &mut self.function,
                    Function::Exponential,
                    "eˣ",
                );
            });


        ui.separator();

        match self.mode {
            Mode::Derivative => self.function_chart.show(ui, self.function),
            Mode::Integral => self.area_chart.show(ui, self.function),
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
