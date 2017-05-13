extern crate gol;
use gol::*;

fn main() {
    let points = [Point { x: 0, y: 0 },
                  Point { x: 1, y: 0 },
                  Point { x: 2, y: 0 }];

    let mut grid = Grid::with_points(points.iter());

    for _ in 0..1001 {
        grid.tick();
    }

    println!("{:?}", grid);
}
