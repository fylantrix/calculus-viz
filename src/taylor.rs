use eframe::egui;
use egui_plot::{Legend, Line, Plot};
use crate::{LOWER, UPPER, Function};

pub struct TaylorSeries {
    iterations: i32,
    location: f64,
}

impl Default for TaylorSeries {
    fn default() -> Self {
        Self { iterations: 1, location: 0.0 }
    }
}

impl TaylorSeries {
    pub fn show(&mut self, ui: &mut egui::Ui, function: Function) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.add(egui::Slider::new(&mut self.iterations, 0..=10).text("iterations"));
            ui.add(egui::Slider::new(&mut self.location, -5.0..=5.0).text("location"));

            let mut original = Vec::with_capacity(300);
            // let mut pointsd = Vec::with_capacity(300);
            let mut taylor_series = Vec::with_capacity(300);

            for i in 0..300 {
                let x = LOWER + (UPPER - LOWER) * i as f64 / 299.0;
                let y = function.eval(x);

                original.push([x, y]);

                // taylor_series.push([
                //     x,
                //     (function.eval(x + self.dx) - y) / self.dx
                // ]);

            }

            let main_line = Line::new("sin(x)", original);
            let derive = Line::new("dy/dx", taylor_series);
            // let del_y = Line::new("y = dy", pointsd);

            Plot::new("line_plot")
                .legend(Legend::default())
                .default_x_bounds(LOWER, UPPER)
                .default_y_bounds(-5.0, 5.0)
                .show(ui, |plot_ui| {
                    plot_ui.line(main_line);
                    plot_ui.line(derive);
                    // plot_ui.line(del_y);
                });
        });
    }
}

fn taylor_series(location: f64, iteration: i32, function: Function, value: &mut f64) {
    if iteration == 0 {
        return;
    }
    let mut kj = 0.0;
    for it in 0..iteration {
        let kj = function.eval(kj);
    }
    *value = kj

}
