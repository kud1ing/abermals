use crate::canvas::Canvas;
use crate::canvas_dom_svg::CanvasSvg;
use crate::cell::Cell;
use crate::error::GameError;
use crate::symbols::box_drawing::{
    double_bl, double_br, double_horizontal, double_tl, double_tr, double_vertical,
};
use crate::symbols::{block, dot, hash, heart, pattern, pattern2, wave};

///
pub(crate) struct Game {
    canvas: CanvasSvg,
}

impl Game {
    pub(crate) fn new() -> Self {
        Game {
            canvas: CanvasSvg::new("canvas"),
        }
    }

    /// Runs the game.
    pub(crate) fn run(&mut self) -> Result<(), GameError> {
        /*
        self.canvas.put_horizontal_line(
            10,
            15,
            9,
            Cell::new(CLASS_FG2, CLASS_BG2, Content::Character('═')),
        )?;
        */

        // Sea.
        self.canvas
            .put_rectangle(0, 30, 0, 20, Cell::new(wave("wave"), "#06f", Some("#039")))?;

        // Grass.
        self.canvas.put_rectangle(
            5,
            13,
            4,
            10,
            Cell::new(pattern2("pattern2"), "#0f0", Some("#060")),
        )?;

        // Earth
        self.canvas.put_horizontal_line(
            6,
            14,
            11,
            Cell::new(pattern("pattern"), "#966", Some("#300")),
        )?;

        // Building.
        self.canvas
            .put(12, 8, Cell::new(pattern("pattern"), "#999", Some("#333")))?;
        self.canvas
            .put(12, 7, Cell::new(block("block"), "#ccc", None))?;

        self.canvas
            .put(13, 4, Cell::new(dot("dot"), "#f0f", Some("#000")))?;
        self.canvas
            .put(8, 8, Cell::new(heart("heart"), "#f0f", Some("#fff")))?;
        self.canvas
            .put(7, 5, Cell::new(hash("hash"), "#ff0", Some("#f30")))?;

        // Box drawing
        self.canvas.put(
            15,
            12,
            Cell::new(double_bl("double_bl"), "#fff", Some("#333")),
        )?;
        self.canvas.put(
            21,
            12,
            Cell::new(double_br("double_br"), "#fff", Some("#333")),
        )?;
        self.canvas.put(
            15,
            10,
            Cell::new(double_tl("double_tl"), "#fff", Some("#333")),
        )?;
        self.canvas.put(
            15,
            11,
            Cell::new(double_vertical("double_vertical"), "#fff", Some("#333")),
        )?;
        self.canvas.put_horizontal_line(
            16,
            20,
            10,
            Cell::new(double_horizontal("double_horizontal"), "#fff", Some("#333")),
        )?;
        self.canvas.put_horizontal_line(
            16,
            20,
            12,
            Cell::new(double_horizontal("double_horizontal"), "#fff", Some("#333")),
        )?;
        self.canvas.put(
            21,
            10,
            Cell::new(double_tr("double_tr"), "#fff", Some("#333")),
        )?;
        self.canvas.put(
            21,
            11,
            Cell::new(double_vertical("double_vertical"), "#fff", Some("#333")),
        )?;

        Ok(())
    }
}
