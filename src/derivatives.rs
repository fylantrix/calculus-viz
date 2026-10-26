use eframe::egui;
use egui_plot::{Legend, Line, Plot, PlotPoints};

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
    pub fn show(&mut self, ui: &mut egui::Ui) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.add(egui::Slider::new(&mut self.dx, 0.0..=2.0).text("dx"));
            ui.checkbox(&mut self.show_ans, "Show answer function");

            let mut points1 = Vec::with_capacity(300);
            let mut pointsd = Vec::with_capacity(300);
            let mut points2 = Vec::with_capacity(300);
            let mut points_cos = Vec::with_capacity(300);


            for i in 0..300 {
                let x = -10.0 + 20.0 * i as f64 / 299.0;

                let y1 = x.sin();
                let dy = (x + self.dx).sin();
                let yc = x.cos();
                let y2 = (dy - y1) / self.dx;

                points1.push([x, y1]);
                pointsd.push([x, dy]);
                points2.push([x, y2]);
                points_cos.push([x, yc]);
            }

            let main_line = Line::new("sin(x)", points1);
            let derive = Line::new("dy1/dx", points2);
            let proof = Line::new("cos(x)", points_cos);
            let del_y = Line::new("y = dy", pointsd);

            Plot::new("line_plot")
                .legend(Legend::default())
                .default_x_bounds(-10.0, 10.0)
                .default_y_bounds(-10.0, 10.0)
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
