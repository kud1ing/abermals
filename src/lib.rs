mod canvas;
mod symbols;

pub use canvas::{Canvas, Character};
pub use symbols::*;

pub const NUMBER_OF_COLUMNS: usize = 9;
pub const NUMBER_OF_ROWS: usize = 15;

///
pub type Row = &'static [bool; NUMBER_OF_COLUMNS];
pub type Symbol = &'static [Row; NUMBER_OF_ROWS];
