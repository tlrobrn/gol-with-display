extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate gol;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
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
    view_ratio: f64,
    elapsed: f64,
    generation: f64,
    rate: f64,
}

impl App {
    fn new(open_gl: glutin_window::OpenGL, window_width: i64, window_height: i64) -> Self {
        let view_ratio = 0.1;

        let view = View {
            top_left: Point { x: 0, y: 0 },
            bottom_right: Point {
                x: (window_width as f64 * view_ratio) as i64,
                y: (window_height as f64 * view_ratio) as i64,
            },
        };

        App {
            gl: GlGraphics::new(open_gl),
            grid: Grid::random(view.top_left, view.bottom_right),
            view_ratio: view_ratio,
            view: view,
            elapsed: 0.0,
            generation: 0.0,
            rate: 10.0,
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        const WHITE: [f32; 4] = [1.0; 4];

        let width = 1.0 / self.view_ratio;
        let square = rectangle::square(0.0, 0.0, width);
        let view_iter = self.view.into_iter();
        let grid: &Grid = &self.grid;

        self.gl
            .draw(args.viewport(), move |c, gl| {
                // Clear the screen.
                clear(WHITE, gl);

                for point in view_iter {
                    match grid.age_of_point(&point) {
                        Some(age) => {
                            let x = point.x as f64;
                            let y = point.y as f64;
                            let transform = c.transform.trans(x * width, y * width);
                            let shade_adjustment = 0.01 * age as f32;
                            let color = [0.0, 0.0, 0.0, 0.1 + shade_adjustment];

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
                x: (new_width as f64 * self.view_ratio) as i64,
                y: (new_height as f64 * self.view_ratio) as i64,
            },
        };
    }
}

fn main() {
    let opengl = OpenGL::V3_2;
    let window_width = 640;
    let window_height = 480;

    let mut window: Window = WindowSettings::new("gol",
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
        e.resize(|w, h| app.resize(w, h));
    }
}
