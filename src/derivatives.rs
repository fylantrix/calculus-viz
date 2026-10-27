use eframe::egui;
use egui_plot::{Legend, Line, Plot, PlotPoints};
use crate::{LOWER, UPPER, Function};

pub struct DerivativeChart {
    dx: f64,
    show_ans: bool,
}

impl Default for DerivativeChart {
    fn default() -> Self {
        Self { dx: 1.0, show_ans: false }
    }
}

impl DerivativeChart {
    pub fn show(&mut self, ui: &mut egui::Ui, function: Function) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.add(egui::Slider::new(&mut self.dx, 0.0..=2.0).text("dx"));
            ui.checkbox(&mut self.show_ans, "Show exact derivative");

            let mut original = Vec::with_capacity(300);
            // let mut pointsd = Vec::with_capacity(300);
            let mut numeric_derivative = Vec::with_capacity(300);
            let mut exact_derivative = Vec::with_capacity(300);


            for i in 0..300 {
                let x = LOWER + (UPPER - LOWER) * i as f64 / 299.0;
                let y = function.eval(x);

                original.push([x, y]);

                numeric_derivative.push([
                    x,
                    (function.eval(x + self.dx) - y) / self.dx
                ]);

                exact_derivative.push([x, function.derivative(x)]);
            }

            let main_line = Line::new("sin(x)", original);
            let derive = Line::new("dy/dx", numeric_derivative);
            let proof = Line::new("cos(x)", exact_derivative);
            // let del_y = Line::new("y = dy", pointsd);

            Plot::new("line_plot")
                .legend(Legend::default())
                .default_x_bounds(LOWER, UPPER)
                .default_y_bounds(-5.0, 5.0)
                .show(ui, |plot_ui| {
                    plot_ui.line(main_line);
                    plot_ui.line(derive);
                    if self.show_ans {
                        plot_ui.line(proof);
                    }
                    // plot_ui.line(del_y);
                });
        });
    }
}
