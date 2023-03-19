use dame::brett::{SpielBrett, Feld, Spieler, Position};
use minifb::{Window, WindowOptions, Key, MouseMode};

const FELD_SIZE: usize = 100;
const SIZE: usize = FELD_SIZE * SpielBrett::SIZE;

type Color = u32;

const fn rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

const BLACK: Color = rgb(0, 0, 0);
const WHITE: Color = rgb(255, 255, 255);
const LIGHT_BLUE: Color = rgb(24, 197, 245);
const DARK_BLUE: Color = rgb(16, 46, 235);
const LIGHT_RED: Color = rgb(247, 35, 77);
const DARK_RED: Color = rgb(130, 3, 28);

struct Application {
    brett: SpielBrett,
    window: Window,
    buffer: Vec<Color>,
}

fn get_feld_color(feld: Feld) -> Color {
    match feld {
        Feld::Leer => BLACK,
        Feld::Stein(Spieler::Mensch) => LIGHT_RED,
        Feld::Stein(Spieler::Computer) => LIGHT_BLUE,
        Feld::Dame(Spieler::Mensch) => DARK_RED,
        Feld::Dame(Spieler::Computer) => DARK_BLUE,
    }
}

impl Application {
    fn open() -> Option<Self> {
        Some(Application {
            brett: SpielBrett::parse(concat!(
                "m m m m \n",
                " m m m m\n",
                "m m m m \n",
                " _ _ _ _\n",
                "_ _ _ _ \n",
                " c c c c\n",
                "c c c c \n",
                " c c c c",
            )).unwrap(),
            buffer: vec![0; SIZE * SIZE],
            window: {
                let mut window = match Window::new("Dame", SIZE, SIZE, WindowOptions::default()) {
                    Ok(window) => window,
                    Err(_) => return None,
                };

                window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

                window
            },
        })
    }

    fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.buffer[y*SIZE+x] = color;
    }

    fn draw_rect(&mut self, x_start: usize, y_start: usize, width: usize, height: usize, color: Color) {
        for x in x_start..x_start+width {
            for y in y_start..y_start+height {
                self.set_pixel(x, y, color);
            }
        }
    }

    fn draw(&mut self) {
        for zeile in 0..SpielBrett::SIZE {
            for spalte in 0..SpielBrett::SIZE {
                let position = Position { spalte, zeile };
                if !position.valid() {
                    self.draw_rect(spalte*FELD_SIZE, zeile*FELD_SIZE, FELD_SIZE, FELD_SIZE, WHITE);
                    continue;
                }
                self.draw_rect(spalte*FELD_SIZE, zeile*FELD_SIZE, FELD_SIZE, FELD_SIZE, BLACK);
                self.draw_rect(spalte*FELD_SIZE+15, zeile*FELD_SIZE+15, FELD_SIZE-30, FELD_SIZE-30, get_feld_color(self.brett.get(position)));
            }
        }
    }

    fn handle_input(&mut self) {
        if self.window.is_key_released(Key::Space) {
            self.brett = self.brett.calculate_best_next_move(7);
        }

        if let Some((mouse_x, mouse_y)) = self.window.get_mouse_pos(MouseMode::Discard) {
            let spalte = mouse_x as usize / FELD_SIZE;
            let zeile = mouse_y as usize / FELD_SIZE;
            let position = Position { spalte, zeile };
            if !position.valid() {
                return;
            }
            if self.window.is_key_released(Key::Backspace) {
                self.brett.set(position, Feld::Leer);
            } else if self.window.is_key_released(Key::M) {
                if self.window.is_key_down(Key::LeftShift) {
                    self.brett.set(position, Feld::Dame(Spieler::Mensch));
                } else {
                    self.brett.set(position, Feld::Stein(Spieler::Mensch));
                }
            } else if self.window.is_key_released(Key::C) {
                if self.window.is_key_down(Key::LeftShift) {
                    self.brett.set(position, Feld::Dame(Spieler::Computer));
                } else {
                    self.brett.set(position, Feld::Stein(Spieler::Computer));
                }
            }
        }
    }

    fn update_loop(mut self) {
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            self.handle_input();

            self.draw();

            self.window.update_with_buffer(&self.buffer, SIZE, SIZE).unwrap();
        }
    }
}

fn main() {
    match Application::open() {
        Some(app) => app.update_loop(),
        None => {},
    }
}
