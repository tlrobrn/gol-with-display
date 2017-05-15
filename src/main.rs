extern crate piston;
extern crate graphics;
extern crate piston_window;
extern crate opengl_graphics;
extern crate gol;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use piston_window::*;
use opengl_graphics::{GlGraphics, OpenGL};
use gol::*;

#[derive(Copy, Clone, Debug)]
struct View {
    top_left: Point,
    bottom_right: Point,
}

impl<'a> IntoIterator for &'a View {
    type Item = Point;
    type IntoIter = ViewIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ViewIterator {
            view: self,
            at: self.top_left,
        }
    }
}

struct ViewIterator<'a> {
    view: &'a View,
    at: Point,
}

impl<'a> Iterator for ViewIterator<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let next_point = if self.at.x < self.view.bottom_right.x {
            self.at + Point { x: 1, y: 0 }
        } else {
            Point {
                x: self.view.top_left.x,
                y: self.at.y + 1,
            }
        };

        if next_point.y < self.view.bottom_right.y {
            self.at = next_point;
            Some(next_point)
        } else {
            None
        }
    }
}

struct App {
    gl: GlGraphics,
    grid: Grid,
    view: View,
    point_width: f64,
    elapsed: f64,
    generation: f64,
    rate: f64,
    mouse_down: bool,
}

impl App {
    fn new(open_gl: piston_window::OpenGL, window_width: i64, window_height: i64) -> Self {
        let point_width = 10.0;

        let view = View {
            top_left: Point { x: 0, y: 0 },
            bottom_right: Point {
                x: (window_width as f64 / point_width) as i64,
                y: (window_height as f64 / point_width) as i64,
            },
        };

        App {
            gl: GlGraphics::new(open_gl),
            grid: Grid::random(view.top_left, view.bottom_right),
            point_width: point_width,
            view: view,
            elapsed: 0.0,
            generation: 0.0,
            rate: 10.0,
            mouse_down: false,
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        const WHITE: [f32; 4] = [1.0; 4];

        let width = self.point_width;
        let square = rectangle::square(0.0, 0.0, width);
        let base_point = self.view.top_left;
        let view_iter = self.view.into_iter();
        let grid: &Grid = &self.grid;

        self.gl
            .draw(args.viewport(), move |c, gl| {
                // Clear the screen.
                clear(WHITE, gl);

                for point in view_iter {
                    match grid.age_of_point(&point) {
                        Some(age) => {
                            let x = (point.x - base_point.x) as f64;
                            let y = (point.y - base_point.y) as f64;
                            let transform = c.transform.trans(x * width, y * width);
                            let shade_adjustment = 0.01 * age as f32;
                            let color = [0.0, 0.0, 0.0, 0.15 + shade_adjustment];

                            rectangle(color, square, transform, gl);
                        }
                        None => {}
                    }
                }
            });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.elapsed += args.dt;
        if self.elapsed > self.generation / self.rate {
            self.grid.tick();
            self.generation += 1.0;
        }
    }

    fn resize(&mut self, new_width: u32, new_height: u32) {
        self.view = View {
            top_left: Point { x: 0, y: 0 },
            bottom_right: Point {
                x: (new_width as f64 / self.point_width) as i64,
                y: (new_height as f64 / self.point_width) as i64,
            },
        };
    }

    fn zoom(&mut self, adjustment: f64) {
        const UPPER_BOUND: f64 = 100.0;
        const LOWER_BOUND: f64 = 1.0;

        let current_width = self.point_width;

        if adjustment > 0.0 {
            self.point_width = UPPER_BOUND.min(self.point_width * 1.5);
        } else {
            self.point_width = LOWER_BOUND.max(self.point_width / 1.5);
        }

        let Point { x, y } = self.view.bottom_right;

        self.view = View {
            top_left: Point { x: 0, y: 0 },
            bottom_right: Point {
                x: (current_width / self.point_width * x as f64) as i64,
                y: (current_width / self.point_width * y as f64) as i64,
            },
        };
    }

    fn shift(&mut self, dx: f64, dy: f64) {
        if !self.mouse_down {
            return;
        }
        let adjustment = Point {
            x: -dx as i64,
            y: -dy as i64,
        };

        self.view = View {
            top_left: self.view.top_left + adjustment,
            bottom_right: self.view.bottom_right + adjustment,
        };
    }
}

fn main() {
    let opengl = OpenGL::V3_2;
    let window_width = 640;
    let window_height = 480;

    let mut window: PistonWindow = WindowSettings::new("gol",
                                                       [window_width as u32, window_height as u32])
            .opengl(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut app = App::new(opengl, window_width, window_height);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        e.render(|r| app.render(r));
        e.update(|u| app.update(u));
        //e.resize(|w, h| app.resize(w, h));
        //e.mouse_scroll(|_dx, dy| app.zoom(dy));
        e.mouse_relative(|dx, dy| app.shift(dx, dy));

        if let Some(button) = e.press_args() {
            match button {
                Button::Mouse(_button) => app.mouse_down = true,
                _ => {}
            };
        }

        if let Some(button) = e.release_args() {
            match button {
                Button::Mouse(_button) => app.mouse_down = false,
                _ => {}
            };
        }
    }
}
