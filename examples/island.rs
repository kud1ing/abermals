use abermals::box_drawing::{
    double_bl, double_br, double_horizontal, double_tl, double_tr, double_vertical,
};
use abermals::{block, dot, hash, heart, pattern, pattern2, wave, Canvas, Character};
use druid_shell::kurbo::Size;
use druid_shell::piet::{Color, Piet};
use druid_shell::Region;
use guiver::{run, Application, SystemEvent};

struct App {
    canvas: Canvas,
}

impl App {
    fn new() -> Self {
        let mut canvas = Canvas::new(80, 25, 2.0, Color::rgb8(0, 0, 0));

        let id_block = canvas.add_symbol(block());
        let id_dot = canvas.add_symbol(dot());
        let id_double_bl = canvas.add_symbol(double_bl());
        let id_double_br = canvas.add_symbol(double_br());
        let id_double_horizontal = canvas.add_symbol(double_horizontal());
        let id_double_tl = canvas.add_symbol(double_tl());
        let id_double_tr = canvas.add_symbol(double_tr());
        let id_double_vertical = canvas.add_symbol(double_vertical());
        let id_hash = canvas.add_symbol(hash());
        let id_heart = canvas.add_symbol(heart());
        let id_pattern = canvas.add_symbol(pattern());
        let id_pattern2 = canvas.add_symbol(pattern2());
        let id_wave = canvas.add_symbol(wave());

        // Water.
        canvas.put_rectangle(
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
        canvas.put_rectangle(
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
        canvas.put_horizontal_line(
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
        canvas.put(
            12,
            8,
            Character::new(
                id_pattern,
                Color::rgb8(150, 150, 150),
                Some(Color::rgb8(50, 50, 50)),
            ),
        );
        canvas.put(
            12,
            7,
            Character::new(id_block, Color::rgb8(150, 150, 150), None),
        );

        // Dot.
        canvas.put(
            13,
            4,
            Character::new(id_dot, Color::rgb8(255, 0, 255), Some(Color::rgb8(0, 0, 0))),
        );

        // Heart.
        canvas.put(
            8,
            8,
            Character::new(
                id_heart,
                Color::rgb8(255, 0, 255),
                Some(Color::rgb8(255, 255, 255)),
            ),
        );

        // Hash.
        canvas.put(
            7,
            5,
            Character::new(
                id_hash,
                Color::rgb8(255, 255, 0),
                Some(Color::rgb8(255, 50, 0)),
            ),
        );

        // Box drawing
        canvas.put(
            15,
            12,
            Character::new(
                id_double_bl,
                Color::rgb8(255, 255, 255),
                Some(Color::rgb8(50, 50, 50)),
            ),
        );
        canvas.put(
            21,
            12,
            Character::new(
                id_double_br,
                Color::rgb8(255, 255, 255),
                Some(Color::rgb8(50, 50, 50)),
            ),
        );
        canvas.put(
            15,
            10,
            Character::new(
                id_double_tl,
                Color::rgb8(255, 255, 255),
                Some(Color::rgb8(50, 50, 50)),
            ),
        );
        canvas.put(
            21,
            10,
            Character::new(
                id_double_tr,
                Color::rgb8(255, 255, 255),
                Some(Color::rgb8(50, 50, 50)),
            ),
        );
        canvas.put(
            15,
            11,
            Character::new(
                id_double_vertical,
                Color::rgb8(255, 255, 255),
                Some(Color::rgb8(50, 50, 50)),
            ),
        );
        canvas.put(
            21,
            11,
            Character::new(
                id_double_vertical,
                Color::rgb8(255, 255, 255),
                Some(Color::rgb8(50, 50, 50)),
            ),
        );
        canvas.put_horizontal_line(
            16,
            20,
            10,
            Character::new(
                id_double_horizontal,
                Color::rgb8(255, 255, 255),
                Some(Color::rgb8(50, 50, 50)),
            ),
        );
        canvas.put_horizontal_line(
            16,
            20,
            12,
            Character::new(
                id_double_horizontal,
                Color::rgb8(255, 255, 255),
                Some(Color::rgb8(50, 50, 50)),
            ),
        );

        App { canvas }
    }
}

impl Application for App {
    fn paint(&mut self, piet: &mut Piet, region: &Region) {
        // Paint the canvas.
        self.canvas.paint(piet, region);
    }

    fn handle_system_event(&mut self, _system_event: &SystemEvent) {}

    fn resize(&mut self, _size: Size) {}
}

pub fn main() {
    run(Box::new(App::new()), "abermals", (800.0, 400.0).into());
}
