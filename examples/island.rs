use abermals::{pattern, pattern2, wave, Canvas, Character, block};
use druid_shell::piet::{Color, Piet};
use druid_shell::Region;
use guiver::{run, Application, UserEvent, WidgetEvent, WidgetId, WidgetManager};

struct App {
    canvas: Canvas,
}

impl App {
    fn new() -> Self {
        App {
            canvas: Canvas::new(80, 25, 2.0, Color::rgb8(0, 0, 0)),
        }
    }
}

impl Application for App {
    fn handle_user_event(&mut self, _user_event: &UserEvent) {}

    fn handle_widget_event(
        &mut self,
        _widget_manager: &mut WidgetManager,
        _widget_id: WidgetId,
        _widget_event: &WidgetEvent,
    ) {
    }

    fn paint(&mut self, piet: &mut Piet, region: &Region) {
        // Paint the canvas.
        self.canvas.paint(piet, region);
    }

    fn setup(&mut self, _widget_manager: &mut WidgetManager) {
        let id_block = self.canvas.add_symbol(block());
        let id_pattern = self.canvas.add_symbol(pattern());
        let id_pattern2 = self.canvas.add_symbol(pattern2());
        let id_wave = self.canvas.add_symbol(wave());

        // Water.
        self.canvas.put_rectangle(
            0,
            30,
            0,
            20,
            Character::new(
                id_wave,
                Color::rgb8(0, 100, 255),
                Some(Color::rgb8(0, 30, 150)),
            ),
        );

        // Grass.
        self.canvas.put_rectangle(
            5,
            13,
            4,
            10,
            Character::new(
                id_pattern2,
                Color::rgb8(0, 255, 0),
                Some(Color::rgb8(0, 100, 0)),
            ),
        );

        // Earth
        self.canvas.put_horizontal_line(
            6,
            14,
            11,
            Character::new(
                id_pattern,
                Color::rgb8(200, 100, 100),
                Some(Color::rgb8(50, 0, 0)),
            ),
        );

        // Building.
        self.canvas.put(
            12,
            8,
            Character::new(
                id_pattern,
                Color::rgb8(150, 150, 150),
                Some(Color::rgb8(50, 50, 50)),
            ),
        );
        self.canvas.put(
            12,
            7,
            Character::new(
                id_block,
                Color::rgb8(150, 150, 150),
                None,
            ),
        );

        /*
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

         */
    }
}

pub fn main() {
    run(Box::new(App::new()), "abermals", (800.0, 400.0).into());
}
