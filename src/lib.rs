use std::f64::consts::PI;

pub const LOWER: f64 = -2.0 * PI;
pub const UPPER: f64 = 2.0 * PI;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Function {
    Sin,
    Cos,
    Square,
    Cube,
    Exponential,
}

impl Function {
    pub fn name(self) -> &'static str {
        match self {
            Self::Sin => "sin(x)",
            Self::Cos => "cos(x)",
            Self::Square => "x²",
            Self::Cube => "x³",
            Self::Exponential => "eˣ",
        }
    }

    pub fn eval(self, x: f64) -> f64 {
        match self {
            Self::Sin => x.sin(),
            Self::Cos => x.cos(),
            Self::Square => x.powi(2),
            Self::Cube => x.powi(3),
            Self::Exponential => x.exp(),
        }
    }

    pub fn derivative(self, x: f64) -> f64 {
        match self {
            Self::Sin => x.cos(),
            Self::Cos => -x.sin(),
            Self::Square => 2.0 * x,
            Self::Cube => 3.0 * x.powi(2),
            Self::Exponential => x.exp(),
        }
    }

    pub fn integral(self, x: f64) -> f64 {
        match self {
            Self::Sin => -1.0 * x.cos(),
            Self::Cos => x.sin(),
            Self::Square => 1.0/3.0 * x.powi(3),
            Self::Cube => 1.0/4.0 * x.powi(4),
            Self::Exponential => x.exp(),
        }
    }
}
