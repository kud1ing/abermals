use abermals::{pattern, pattern2, wave, Canvas, Character, block, dot, heart, hash};
use druid_shell::piet::{Color, Piet};
use druid_shell::Region;
use guiver::{run, Application, UserEvent, WidgetEvent, WidgetId, WidgetManager};
use abermals::box_drawing::{double_bl, double_br, double_horizontal, double_tl, double_tr, double_vertical};

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
        let id_dot = self.canvas.add_symbol(dot());
        let id_double_bl = self.canvas.add_symbol(double_bl());
        let id_double_br = self.canvas.add_symbol(double_br());
        let id_double_horizontal = self.canvas.add_symbol(double_horizontal());
        let id_double_tl = self.canvas.add_symbol(double_tl());
        let id_double_tr = self.canvas.add_symbol(double_tr());
        let id_double_vertical = self.canvas.add_symbol(double_vertical());
        let id_hash = self.canvas.add_symbol(hash());
        let id_heart = self.canvas.add_symbol(heart());
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

        // Dot.
        self.canvas.put(
            13,
            4,
            Character::new(
                id_dot,
                Color::rgb8(255, 0, 255),
                Some(Color::rgb8(0,0,0)),
            ),
        );

        // Heart.
        self.canvas.put(
            8,
            8,
            Character::new(
                id_heart,
                Color::rgb8(255, 0, 255),
                Some(Color::rgb8(255,255,255)),
            ),
        );

        // Hash.
        self.canvas.put(
            7,
            5,
            Character::new(
                id_hash,
                Color::rgb8(255, 255, 0),
                Some(Color::rgb8(255,50,0)),
            ),
        );

        // Box drawing
        self.canvas.put(
            15,
            12,
            Character::new(
                id_double_bl,
                Color::rgb8(255, 255, 255),
                Some(Color::rgb8(50,50,50)),
            ),
        );
        self.canvas.put(
            21,
            12,
            Character::new(
                id_double_br,
                Color::rgb8(255, 255, 255),
                Some(Color::rgb8(50,50,50)),
            ),
        );
        self.canvas.put(
            15,
            10,
            Character::new(
                id_double_tl,
                Color::rgb8(255, 255, 255),
                Some(Color::rgb8(50,50,50)),
            ),
        );
        self.canvas.put(
            21,
            10,
            Character::new(
                id_double_tr,
                Color::rgb8(255, 255, 255),
                Some(Color::rgb8(50,50,50)),
            ),
        );
        self.canvas.put(
            15,
            11,
            Character::new(
                id_double_vertical,
                Color::rgb8(255, 255, 255),
                Some(Color::rgb8(50,50,50)),
            ),
        );
        self.canvas.put(
            21,
            11,
            Character::new(
                id_double_vertical,
                Color::rgb8(255, 255, 255),
                Some(Color::rgb8(50,50,50)),
            ),
        );
        self.canvas.put_horizontal_line(
            16,
            20,
            10,
            Character::new(
                id_double_horizontal,
                Color::rgb8(255, 255, 255),
                Some(Color::rgb8(50,50,50)),
            ),
        );
        self.canvas.put_horizontal_line(
            16,
            20,
            12,
            Character::new(
                id_double_horizontal,
                Color::rgb8(255, 255, 255),
                Some(Color::rgb8(50,50,50)),
            ),
        );
    }
}

pub fn main() {
    run(Box::new(App::new()), "abermals", (800.0, 400.0).into());
}
