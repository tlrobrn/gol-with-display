use std::ops::Add;
use std::collections::HashSet;


#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub struct Point {
    x: i64,
    y: i64,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

const NEIGHBORHOOD_OFFSETS: [Point; 8] = [Point { x: -1, y: 1 },
                                          Point { x: 0, y: 1 },
                                          Point { x: 1, y: 1 },
                                          Point { x: -1, y: 0 },
                                          Point { x: 1, y: 0 },
                                          Point { x: -1, y: -1 },
                                          Point { x: 0, y: -1 },
                                          Point { x: 1, y: -1 }];

pub fn neighbors(point: Point) -> Vec<Point> {
    NEIGHBORHOOD_OFFSETS
        .iter()
        .map(|&offset| point + offset)
        .collect()
}

#[derive(Eq, PartialEq, Debug)]
pub struct Grid {
    cells: HashSet<Point>,
}

impl Grid {
    pub fn new<'a, I>(points: I) -> Self
        where I: Iterator<Item = &'a Point>
    {
        let mut cells = HashSet::new();
        for point in points {
            cells.insert(point.clone());
        }

        Grid { cells }
    }

    pub fn tick(&mut self) -> &Self {
        let mut next_generation = HashSet::new();

        for cell in self.cells.iter() {
            let count = self.count_neighbors(cell);

            if count > 1 && count < 4 {
                next_generation.insert(cell.clone());
            }
        }

        for cell in self.dead_candidates() {
            let count = self.count_neighbors(&cell);

            if count == 3 {
                next_generation.insert(cell);
            }
        }

        self.cells = next_generation;

        self
    }

    fn count_neighbors(&self, point: &Point) -> usize {
        neighbors(*point)
            .iter()
            .fold(0, |acc, point| if self.cells.contains(point) {
                acc + 1
            } else {
                acc
            })
    }

    fn dead_candidates(&self) -> Vec<Point> {
        self.cells
            .iter()
            .flat_map(|cell| neighbors(*cell))
            .filter(|cell| !self.cells.contains(cell))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neighbors_returns_an_array_of_the_surrounding_points() {
        let p = Point { x: 1, y: 5 };
        let expected_result: Vec<Point> = vec![Point { x: 0, y: 6 },
                                               Point { x: 1, y: 6 },
                                               Point { x: 2, y: 6 },
                                               Point { x: 0, y: 5 },
                                               Point { x: 2, y: 5 },
                                               Point { x: 0, y: 4 },
                                               Point { x: 1, y: 4 },
                                               Point { x: 2, y: 4 }];

        assert_eq!(expected_result, neighbors(p));
    }

    #[test]
    fn grid_contains_point_initialized_with() {
        let points = [Point { x: 5, y: 2 }];
        let g = Grid::new(points.iter());

        assert!(g.cells.contains(&points[0]));
        assert_eq!(1, g.cells.len());
    }

    #[test]
    fn grid_tick_will_kill_living_cells_with_less_than_two_neighbors() {
        let points = [Point { x: 0, y: 0 },
                      Point { x: 5, y: 2 },
                      Point { x: 5, y: 3 }];
        let mut g = Grid::new(points.iter());
        g.tick();

        assert_eq!(0, g.cells.len());
    }

    #[test]
    fn grid_tick_will_kill_living_cells_with_more_than_three_neighbors() {
        let points = [Point { x: 0, y: 0 },
                      Point { x: 2, y: 0 },
                      Point { x: 1, y: 1 },
                      Point { x: 1, y: 2 },
                      Point { x: 0, y: 1 }];
        let mut g = Grid::new(points.iter());
        g.cells.insert(Point { x: 0, y: 0 });

        g.tick();
        assert!(!g.cells.contains(&Point { x: 1, y: 1 }));
    }

    #[test]
    fn grid_tick_will_make_a_cell_alive_if_it_has_3_neighbors() {
        let points = [Point { x: 0, y: 1 },
                      Point { x: -1, y: 0 },
                      Point { x: 1, y: 0 }];
        let mut g = Grid::new(points.iter());

        g.tick();

        assert!(g.cells.contains(&Point { x: 0, y: 0 }));
    }

    #[test]
    fn grid_tick_will_preserve_cells_with_2_neighbors() {
        let points = [Point { x: 5, y: 1 },
                      Point { x: 5, y: 2 },
                      Point { x: 5, y: 3 }];
        let mut g = Grid::new(points.iter());
        g.tick();

        assert!(g.cells.contains(&Point { x: 5, y: 2 }));
    }

    #[test]
    fn grid_tick_will_preserve_cells_with_3_neighbors() {
        let points = [Point { x: 5, y: 1 },
                      Point { x: 5, y: 2 },
                      Point { x: 6, y: 2 },
                      Point { x: 5, y: 3 }];
        let mut g = Grid::new(points.iter());
        g.tick();

        assert!(g.cells.contains(&Point { x: 5, y: 2 }));
    }
}
