use crate::cell::Cell;
use crate::error::GameError;

pub trait Canvas {
    /// Clears the cell.
    fn clear(&self, index_x: u32, index_y: u32) -> Result<(), GameError>;

    /// Puts the cell.
    fn put(&mut self, index_x: u32, index_y: u32, cell: Cell) -> Result<(), GameError>;
}
