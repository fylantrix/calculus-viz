use egui::{Color32, Stroke};
use eframe::egui;
use egui_plot::{Polygon, Legend, Line, Plot, PlotPoints};
use crate::{LOWER, UPPER};

pub struct IntegralChart {
    dx: f64,
    show_ans: bool
}

impl Default for IntegralChart {
    fn default() -> Self {
        Self { dx: 1.0, show_ans: false }
    }
}

impl IntegralChart {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.add(egui::Slider::new(&mut self.dx, 0.0..=1.0).text("dx"));
            ui.checkbox(&mut self.show_ans, "Show answer function");

            let mut points1 = Vec::with_capacity(300);
            let mut integral_points = Vec::with_capacity(301);
            let mut points_cos = Vec::with_capacity(300);
            let mut rects = Vec::new();

            for i in 0..300 {
                let x = LOWER + (UPPER - LOWER) * i as f64 / 299.0;

                let y1 = x.sin();
                let yc = -1.0 * x.cos();

                points_cos.push([x, yc]);
                points1.push([x, y1]);
            }

            if self.dx > 0.0 {
                let span = UPPER - LOWER;
                let n_rect = (span / self.dx).ceil() as usize;

                let mut integral = 0.0;
                integral_points.push([LOWER, integral]);

                for i in 0..n_rect {
                    let x = LOWER + i as f64 * self.dx;
                    let width = self.dx.min(UPPER - x);
                    let height = x.sin();

                    rects.push(Polygon::new(
                        "region",
                        vec![
                            [x, 0.0],
                            [x + width, 0.0],
                            [x + width, height],
                            [x, height],
                        ],
                    )
                        .fill_color(Color32::from_rgba_unmultiplied(100, 180, 255, 40))
                        .stroke(Stroke::new(1.5, Color32::LIGHT_BLUE)));


                    integral += height * width;
                    integral_points.push([x + width, integral]);
                }
            }

            let main_line = Line::new("sin(x)", points1);
            let derive = Line::new("integral", integral_points);
            let proof = Line::new("cos(x)", points_cos);

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
                    for rect in rects {
                        plot_ui.polygon(rect);
                    }
                });
        });
    }
}
