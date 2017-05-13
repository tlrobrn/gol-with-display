extern crate gol;
use gol::*;

fn main() {
    let points = [Point { x: 0, y: 0 },
                  Point { x: 1, y: 0 },
                  Point { x: 0, y: 1 },
                  Point { x: 1, y: 1 }];

    let mut grid = Grid::new(points.iter());

    for _ in 0..1000 {
        grid.tick();
    }

    println!("{:?}", grid);
}
