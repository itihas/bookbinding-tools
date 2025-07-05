use clap::ValueEnum;
use std::collections::HashMap;

// in points
const A4_DIMS: (f64, f64) = (595.28, 841.89);
const A5_DIMS: (f64, f64) = (419.53, 595.28);
const A6_DIMS: (f64, f64) = (297.64, 419.53);
const A7_DIMS: (f64, f64) = (209.76, 297.64);

pub struct Dimension {
    pub w: f64,
    pub h: f64,
}

impl Dimension {
    pub fn transpose(self) -> Self {
        Dimension {
            w: self.h,
            h: self.w,
        }
    }
}

impl std::ops::Mul<f64> for Dimension {
    type Output = self::Dimension;

    fn mul(self, rhs: f64) -> Self::Output {
        return Dimension {
            w: self.w * rhs,
            h: self.h * rhs,
        };
    }
}

impl std::ops::Mul for Dimension {
    type Output = self::Dimension;

    fn mul(self, rhs: Self) -> Self::Output {
        return Dimension {
            w: self.w * rhs.w,
            h: self.h * rhs.h,
        };
    }
}

impl std::ops::Div for Dimension {
    type Output = self::Dimension;

    fn div(self, rhs: Self) -> Self::Output {
        return Dimension {
            w: self.w / rhs.w,
            h: self.h / rhs.h,
        };
    }
}

#[derive(ValueEnum, Clone, Debug)]
pub enum PaperSize {
    A4,
    A5,
    A6,
    A7,
    // Custom(f64, f64),
}

impl PaperSize {
    pub fn size(&self) -> Dimension {
        let (w, h) = match self {
            PaperSize::A4 => A4_DIMS,
            PaperSize::A5 => A5_DIMS,
            PaperSize::A6 => A6_DIMS,
            PaperSize::A7 => A7_DIMS,
            // PaperSize::Custom(w, h) => (*w, *h)
        };
        Dimension { w, h }
    }
}

impl From<&str> for PaperSize {
    fn from(s: &str) -> Self {
        match s {
            "A4" => PaperSize::A4,
            "A5" => PaperSize::A5,
            "A6" => PaperSize::A6,
            "A7" => PaperSize::A7,
            _ => PaperSize::A4,
        }
    }
}

#[derive(Debug)]
pub enum Layout {
    Folio,
    Quarto,
}

impl Layout {
    fn transform() {
        todo!()
    }
}

#[derive(Debug)]
pub struct Block {
    pub galley: PaperSize,
    pub layout: Layout,
    pub signature_size: u32,
    pub margins: MarginSet,
}

#[derive(Debug)]
pub struct MarginSet {
    pub printer: f64,
    pub cutting: f64,
    pub binding: f64,
    pub annotation: f64,
}

impl MarginSet {
    fn get_galley_margins() {
        todo!()
    }
    fn get_page_margins() {
        todo!()
    }
}

impl Block {
    fn make_block() {
        todo!()
    }
    fn make_signature() {
        todo!()
    }
    fn make_galley() {
        todo!()
    }
}
