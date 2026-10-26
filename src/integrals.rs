use std::f64::consts::PI;

use eframe::egui;
use egui_plot::{Legend, Line, Plot, PlotPoints};


pub struct IntegralChart {
    dx: f64,
    start: f64,
    end: f64,
    show_ans: bool
}

impl Default for IntegralChart {
    fn default() -> Self {
        Self { dx: 1.0, start: -PI, end: PI, show_ans: false }
    }
}

impl IntegralChart {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.add(egui::Slider::new(&mut self.dx, 0.0001..=1.0).text("dx"));
            ui.checkbox(&mut self.show_ans, "Show answer function");

            let mut points1 = Vec::with_capacity(300);
            let mut pointsd = Vec::with_capacity(300);
            let mut points2 = Vec::with_capacity(300);
            let mut points_cos = Vec::with_capacity(300);


            for i in 0..300 {
                let x = (-2.0 * PI) + (4.0 * PI) * i as f64 / 299.0;

                let y1 = x.sin();
                let yc = -1.0 * x.cos();

                points_cos.push([x, yc]);
                points1.push([x, y1]);
            }

            let wl = self.end - self.start;
            let n_rect: i32 = (wl / self.dx) as i32;
            // let width_rect =  

            for i in 0..n_rect {
                let x = (-2.0 * PI) + (4.0 * PI) * i as f64 / n_rect as f64;

                let dy = x.sin() * self.dx;

                pointsd.push([x, dy]);

                let y2 = {
                    let mut sd: f64 = 0.0;
                    for (_, val) in pointsd.iter().enumerate() {
                        sd += val[1];
                    }
                    sd
                };
                points2.push([x, y2]);
            }


            let main_line = Line::new("sin(x)", points1);
            let derive = Line::new("integral", points2);
            let proof = Line::new("cos(x)", points_cos);
            let del_y = Line::new("y = dy", pointsd);

            Plot::new("line_plot")
                .legend(Legend::default())
                .default_x_bounds(-PI, PI)
                .default_y_bounds(-5.0, 5.0)
                .show(ui, |plot_ui| {
                    // plot_ui.line(main_line);
                    plot_ui.line(derive);
                    if self.show_ans {
                        plot_ui.line(proof);
                    }
                    // plot_ui.line(del_y);
                });
        });
    }
}
