use crate::canvas::Canvas;
use crate::cell::Cell;
use crate::error::GameError;
use crate::symbol::{Symbol, NUMBER_OF_COLUMNS, NUMBER_OF_ROWS};
use std::collections::HashSet;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, Window};

const SVG_NAMESPACE: &str = "http://www.w3.org/2000/svg";
const XLINK_NAMESPACE: &str = "http://www.w3.org/1999/xlink";

///
pub struct CanvasSvg {
    cell_height: f32,
    cell_width: f32,
    dom_identifier: String,
    pixel_height: f32,
    pixel_width: f32,
    registered_symbols: HashSet<String>,
}

/// Creates a string that can be used as the class string for the DOM element.
fn class_name_cell(index_x: u32, index_y: u32) -> String {
    format!("x{}y{}", index_x, index_y)
}

/// Tries to return the browser window.
fn window() -> Result<Window, GameError> {
    // Try to get the browser window.
    web_sys::window()
        .ok_or_else(|| GameError::Unknown("Could not get the browser window".to_string()))
}

/// Tries to return the window document.
fn window_document(window: &Window) -> Result<Document, GameError> {
    // Try to get the window document.
    window
        .document()
        .ok_or_else(|| GameError::Unknown("Could not get the browser window document".to_string()))
}

impl CanvasSvg {
    pub fn new(dom_identifier: &str) -> Self {
        let pixel_height = 2.0;
        let pixel_width = 2.0;
        let cell_height = pixel_height * NUMBER_OF_ROWS as f32;
        let cell_width = pixel_width * NUMBER_OF_COLUMNS as f32;

        CanvasSvg {
            cell_height,
            cell_width,
            dom_identifier: dom_identifier.to_string(),
            pixel_height,
            pixel_width,
            registered_symbols: HashSet::new(),
        }
    }

    /// Tries to add the given graphic.
    fn add_symbol(
        &mut self,
        window_document: &Document,
        class_name_cell: &str,
        fill: &str,
        x: f32,
        y: f32,
        symbol: Symbol,
    ) -> Result<(), GameError> {
        // Try to get the SVG canvas element.
        let svg_canvas_element = self.svg_canvas_element(window_document)?;

        // The given symbol has not yet been registered.
        if !self.registered_symbols.contains(symbol.identifier) {
            // Create the `symbol` SVG element.
            let svg_element_symbol: Element = create_svg_element(window_document, "symbol")?;

            // Set the identifier.
            set_svg_element_attribute(&svg_element_symbol, "id", symbol.identifier)?;

            // Set the dimensions.
            set_svg_element_attribute(
                &svg_element_symbol,
                "width",
                &format!("{}", self.cell_width),
            )?;
            set_svg_element_attribute(
                &svg_element_symbol,
                "height",
                &format!("{}", self.cell_height),
            )?;

            /*
            set_svg_element_attribute(
                &svg_element_symbol,
                "viewBox",
                &format!("0 0 {} {}", self.cell_width, self.cell_height),
            )?;
            */

            for (pixel_y, row) in symbol.rows.iter().enumerate() {
                for (pixel_x, pixel_is_set) in row.iter().enumerate() {
                    if *pixel_is_set {
                        // Create a `rect` SVG element for the current pixel.
                        let svg_element_pixel: Element =
                            create_svg_element(window_document, "rect")?;

                        // Set the pixel dimensions.
                        set_svg_element_attribute(
                            &svg_element_pixel,
                            "width",
                            &format!("{}", self.pixel_width),
                        )?;
                        set_svg_element_attribute(
                            &svg_element_pixel,
                            "height",
                            &format!("{}", self.pixel_height),
                        )?;

                        // Set the pixel position
                        set_svg_element_attribute(
                            &svg_element_pixel,
                            "x",
                            &format!("{}", self.pixel_width * pixel_x as f32),
                        )?;
                        set_svg_element_attribute(
                            &svg_element_pixel,
                            "y",
                            &format!("{}", self.pixel_height * pixel_y as f32),
                        )?;

                        // Add the `rect` to the `symbol`.
                        svg_element_symbol
                            .append_child(&svg_element_pixel)
                            .map_err(|_| {
                                GameError::Unknown("Can not append `symbol` to parent.".to_string())
                            })?;
                    }
                }
            }

            // Add the SVG element to the SVG canvas.
            svg_canvas_element
                .append_child(&svg_element_symbol)
                .map_err(|_| {
                    GameError::Unknown("Can not append `symbol` to parent.".to_string())
                })?;

            // Mark the symbol as registered.
            self.registered_symbols
                .insert(symbol.identifier.to_string());
        }

        // Use the symbol in the SVG element.
        {
            // Create the `use` SVG element.
            let svg_element: Element = create_svg_element(window_document, "use")?;

            set_svg_element_attribute_namespace(
                &svg_element,
                XLINK_NAMESPACE,
                "href",
                &format!("#{}", symbol.identifier),
            )?;

            // Set the class.
            set_svg_element_attribute(&svg_element, "class", class_name_cell)?;
            // Set the style.
            set_svg_element_attribute(&svg_element, "style", &format!("fill:{};", fill))?;

            // Set the position
            set_svg_element_attribute(&svg_element, "x", &format!("{}", x))?;
            set_svg_element_attribute(&svg_element, "y", &format!("{}", y))?;

            // Add the SVG element to the SVG canvas.
            svg_canvas_element
                .append_child(&svg_element)
                .map_err(|_| GameError::Unknown("Can not append `use` to parent.".to_string()))?;
        }

        Ok(())
    }

    /// Tries to add a background rectangle.
    fn add_background(
        &self,
        window_document: &Document,
        class_name_cell: &str,
        fill: &str,
        x: f32,
        y: f32,
    ) -> Result<(), GameError> {
        // Try to get the SVG canvas element.
        let svg_canvas_element = self.svg_canvas_element(window_document)?;

        // Create the `rect` SVG element.
        let svg_element: Element = create_svg_element(window_document, "rect")?;

        // Set the attributes.
        {
            // Set the class.
            set_svg_element_attribute(&svg_element, "class", class_name_cell)?;
            // Set the style.
            set_svg_element_attribute(&svg_element, "style", &format!("fill:{};", fill))?;

            // Set the dimensions.
            set_svg_element_attribute(&svg_element, "width", &format!("{}", self.cell_width))?;
            set_svg_element_attribute(&svg_element, "height", &format!("{}", self.cell_height))?;

            // Set the position
            set_svg_element_attribute(&svg_element, "x", &format!("{}", x))?;
            set_svg_element_attribute(&svg_element, "y", &format!("{}", y))?;
        }

        // Add the SVG element to the SVG canvas.
        svg_canvas_element
            .append_child(&svg_element)
            .map_err(|_| GameError::Unknown("Can not append `rect` to parent.".to_string()))?;

        Ok(())
    }

    ///
    fn clear_cell_internal(
        &self,
        window_document: &Document,
        class_name_cell: &str,
    ) -> Result<(), GameError> {
        // Query the foreground and background elements for that cell.
        let node_list = window_document
            .query_selector_all(&class_name_cell)
            .map_err(|_| GameError::Unknown("Could not query the cell.".to_string()))?;

        // Iterate over the elements.
        for index in 0..node_list.length() {
            if let Some(node) = node_list.item(index) {
                if let Some(element) = node.dyn_ref::<Element>() {
                    element.remove();
                }
            }
        }

        Ok(())
    }

    /// Puts a rectangle with the given cell.
    pub fn put_rectangle(
        &mut self,
        start_index_x: u32,
        end_index_x: u32,
        start_index_y: u32,
        end_index_y: u32,
        cell: Cell,
    ) -> Result<(), GameError> {
        let window = window()?;
        let window_document = window_document(&window)?;

        for index_y in start_index_y..=end_index_y {
            for index_x in start_index_x..=end_index_x {
                self.put_internal(&window_document, index_x, index_y, cell.clone())?;
            }
        }

        Ok(())
    }

    /// Puts a horizontal cell with the given cell.
    pub fn put_horizontal_line(
        &mut self,
        start_index_x: u32,
        end_index_x: u32,
        index_y: u32,
        cell: Cell,
    ) -> Result<(), GameError> {
        let window = window()?;
        let window_document = window_document(&window)?;

        for index_x in start_index_x..=end_index_x {
            self.put_internal(&window_document, index_x, index_y, cell.clone())?;
        }

        Ok(())
    }

    /// Puts the cell.
    fn put_internal(
        &mut self,
        window_document: &Document,
        index_x: u32,
        index_y: u32,
        cell: Cell,
    ) -> Result<(), GameError> {
        let class_name_cell = class_name_cell(index_x, index_y);

        // Clear the cell.
        self.clear_cell_internal(&window_document, &class_name_cell);

        let x = self.cell_width * index_x as f32;
        let y = self.cell_height * index_y as f32;

        // A background style is given.
        if let Some(fill_background) = cell.fill_background {
            // Add the background rectangle.
            self.add_background(&window_document, &class_name_cell, &fill_background, x, y)?;
        }

        // Add the symnbol.
        self.add_symbol(
            &window_document,
            &class_name_cell,
            &cell.fill_foreground,
            x,
            y,
            cell.symbol,
        )?;

        Ok(())
    }

    /// Tries to return the SVG canvas element.
    fn svg_canvas_element(&self, window_document: &Document) -> Result<Element, GameError> {
        window_document
            .get_element_by_id(&self.dom_identifier)
            .ok_or_else(|| GameError::Unknown("Could not get SVG element".to_string()))
    }
}

impl Canvas for CanvasSvg {
    fn clear(&self, index_x: u32, index_y: u32) -> Result<(), GameError> {
        let window = window()?;
        let window_document = window_document(&window)?;

        let class_name_cell = class_name_cell(index_x, index_y);

        self.clear_cell_internal(&window_document, &class_name_cell)
    }

    fn put(&mut self, index_x: u32, index_y: u32, cell: Cell) -> Result<(), GameError> {
        let window = window()?;
        let window_document = window_document(&window)?;

        self.put_internal(&window_document, index_x, index_y, cell)
    }
}

///
fn create_svg_element(window_document: &Document, tag: &str) -> Result<Element, GameError> {
    window_document
        .create_element_ns(Some(SVG_NAMESPACE), tag)
        .map_err(|_| GameError::Unknown(format!("Could not create `{}` element.", tag)))
}

///
fn set_svg_element_attribute(
    svg_element: &Element,
    key: &str,
    value: &str,
) -> Result<(), GameError> {
    svg_element
        .set_attribute(key, value)
        .map_err(|_| GameError::Unknown(format!("Could not set element attribute \"{}\".", key)))?;

    Ok(())
}

///
fn set_svg_element_attribute_namespace(
    svg_element: &Element,
    namespace: &str,
    key: &str,
    value: &str,
) -> Result<(), GameError> {
    svg_element
        .set_attribute_ns(Some(namespace), key, value)
        .map_err(|_| {
            GameError::Unknown(format!(
                "Could not set element attribute \"{}:{}\".",
                namespace, key
            ))
        })?;

    Ok(())
}
