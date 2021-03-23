use crate::symbol::Symbol;

///
#[derive(Clone)]
pub struct Cell {
    pub(crate) fill_foreground: String,
    pub(crate) fill_background: Option<String>,
    pub(crate) symbol: Symbol,
}

impl Cell {
    pub fn new(symbol: Symbol, fill_foreground: &str, fill_background: Option<&str>) -> Self {
        Cell {
            fill_background: fill_background.map(|s| s.to_string()),
            fill_foreground: fill_foreground.to_string(),
            symbol,
        }
    }
}
