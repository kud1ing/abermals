use crate::{Symbol, NUMBER_OF_COLUMNS, NUMBER_OF_ROWS};
use druid_shell::kurbo::Rect;
use druid_shell::piet::{Color, Piet, RenderContext};
use druid_shell::Region;
use std::collections::HashMap;

/// A character in the canvas.
#[derive(Clone)]
pub struct Character {
    pub(crate) background: Option<Color>,
    pub(crate) foreground: Color,
    pub(crate) symbol_index: usize,
}

impl Character {
    ///
    pub fn new(symbol_index: usize, foreground: Color, background: Option<Color>) -> Self {
        Character {
            background,
            foreground,
            symbol_index,
        }
    }
}

// =================================================================================================

///
pub struct Canvas {
    characters: HashMap<(usize, usize), Character>,
    character_height: f64,
    character_width: f64,
    color: Color,
    height: usize,
    pixel_height: f64,
    pixel_width: f64,
    symbols: Vec<Symbol>,
    width: usize,
}

impl Canvas {
    ///
    pub fn new(width: usize, height: usize, pixel_size: f64, color: Color) -> Self {
        let pixel_height = pixel_size;
        let pixel_width = pixel_size;
        let cell_height = pixel_height * NUMBER_OF_ROWS as f64;
        let cell_width = pixel_width * NUMBER_OF_COLUMNS as f64;

        Canvas {
            characters: HashMap::new(),
            character_height: cell_height,
            character_width: cell_width,
            color,
            height,
            pixel_height,
            pixel_width,
            symbols: vec![],
            width,
        }
    }

    /// Add the given symbol and return its index.
    pub fn add_symbol(&mut self, symbol: Symbol) -> usize {
        let symbol_index = self.symbols.len();
        self.symbols.push(symbol);
        symbol_index
    }

    /// Paints the canvas.
    pub fn paint(&mut self, piet: &mut Piet, _region: &Region) {
        for row in 0..self.height {
            let character_offset_y = row as f64 * self.character_height;

            for column in 0..self.width {
                let character_offset_x = column as f64 * self.character_width;

                // There is a character.
                if let Some(character) = self.characters.get(&(column, row)) {
                    // The cell has a background.
                    let background_color = if let Some(color) = &character.background {
                        // Use the cell's background color.
                        color
                    }
                    // The cell has no background.
                    else {
                        // Use the canvas's background color.
                        &self.color
                    };

                    // Draw the cell's background.
                    piet.fill(
                        Rect::new(
                            character_offset_x,
                            character_offset_y,
                            character_offset_x + self.character_width,
                            character_offset_y + self.character_height,
                        ),
                        background_color,
                    );

                    // The symbol could be found.
                    if let Some(symbol) = self.symbols.get(character.symbol_index) {
                        // Draw the symbol.
                        for pixel_column in 0..NUMBER_OF_COLUMNS {
                            let pixel_x =
                                character_offset_x + pixel_column as f64 * self.pixel_width;

                            for pixel_row in 0..NUMBER_OF_ROWS {
                                let pixel_y =
                                    character_offset_y + pixel_row as f64 * self.pixel_height;

                                let pixel_is_not_transparant = symbol[pixel_row][pixel_column];

                                if pixel_is_not_transparant {
                                    // Draw the pixel.
                                    piet.fill(
                                        Rect::new(
                                            pixel_x,
                                            pixel_y,
                                            pixel_x + self.pixel_width,
                                            pixel_y + self.pixel_height,
                                        ),
                                        &character.foreground,
                                    );
                                }
                            }
                        }
                    }
                    // The symbol could not be found.
                    else {
                        assert!(false, "Symbol not found")
                    }
                }
                // There is no cell.
                else {
                    // Draw the canvas background.
                    piet.fill(
                        Rect::new(
                            character_offset_x,
                            character_offset_y,
                            character_offset_x + self.character_width,
                            character_offset_y + self.character_height,
                        ),
                        &self.color,
                    );
                }
            }
        }
    }

    /// Puts a character.
    pub fn put(&mut self, index_x: usize, index_y: usize, cell: Character) {
        self.characters.insert((index_x, index_y), cell);
    }

    /// Puts a rectangle, filled with the given character.
    pub fn put_rectangle(
        &mut self,
        start_index_x: usize,
        end_index_x: usize,
        start_index_y: usize,
        end_index_y: usize,
        character: Character,
    ) {
        for index_y in start_index_y..=end_index_y {
            for index_x in start_index_x..=end_index_x {
                self.put(index_x, index_y, character.clone());
            }
        }
    }

    /// Puts a horizontal line, filled with the given character.
    pub fn put_horizontal_line(
        &mut self,
        start_index_x: usize,
        end_index_x: usize,
        index_y: usize,
        character: Character,
    ) {
        for index_x in start_index_x..=end_index_x {
            self.put(index_x, index_y, character.clone());
        }
    }

    /// Removes a cell.
    pub fn remove(&mut self, position: (usize, usize)) {
        self.characters.remove(&position);
    }
}
