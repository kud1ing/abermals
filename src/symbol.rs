pub const NUMBER_OF_COLUMNS: usize = 9;
pub const NUMBER_OF_ROWS: usize = 15;

///
pub type Row = &'static [bool; NUMBER_OF_COLUMNS];

///
#[derive(Clone)]
pub struct Symbol {
    pub identifier: &'static str,
    pub rows: &'static [Row; NUMBER_OF_ROWS],
}
