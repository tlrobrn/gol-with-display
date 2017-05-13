use std::ops::Add;
use std::collections::HashMap;


#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub struct Point {
    pub x: i64,
    pub y: i64,
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

#[derive(Debug)]
pub struct Grid {
    cells: HashMap<Point, u64>,
    generation: u64,
}

impl Grid {
    pub fn empty() -> Self {
        Grid {
            cells: HashMap::new(),
            generation: 0,
        }
    }

    pub fn with_points<'a, I>(points: I) -> Self
        where I: Iterator<Item = &'a Point>
    {
        let mut cells = HashMap::new();
        for point in points {
            cells.insert(point.clone(), 0);
        }

        Grid {
            cells,
            generation: 0,
        }
    }

    pub fn age_of_point(&self, point: &Point) -> Option<u64> {
        self.cells.get(point).map(|birth| self.generation - birth)
    }

    pub fn tick(&mut self) -> &Self {
        self.generation += 1;
        let mut next_generation = HashMap::new();

        for (cell, generation) in self.cells.iter() {
            let count = self.count_neighbors(cell);

            if count > 1 && count < 4 {
                next_generation.insert(cell.clone(), generation.clone());
            }
        }

        for cell in self.dead_candidates() {
            let count = self.count_neighbors(&cell);

            if count == 3 {
                next_generation.insert(cell, self.generation.clone());
            }
        }

        self.cells = next_generation;
        self
    }

    fn count_neighbors(&self, point: &Point) -> usize {
        neighbors(*point)
            .iter()
            .fold(0, |acc, point| if self.cells.contains_key(point) {
                acc + 1
            } else {
                acc
            })
    }

    fn dead_candidates(&self) -> Vec<Point> {
        self.cells
            .iter()
            .flat_map(|(cell, _gen)| neighbors(*cell))
            .filter(|cell| !self.cells.contains_key(cell))
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
        let g = Grid::with_points(points.iter());

        assert!(g.cells.contains_key(&points[0]));
        assert_eq!(1, g.cells.len());
    }

    #[test]
    fn grid_tick_will_kill_living_cells_with_less_than_two_neighbors() {
        let points = [Point { x: 0, y: 0 },
                      Point { x: 5, y: 2 },
                      Point { x: 5, y: 3 }];
        let mut g = Grid::with_points(points.iter());
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
        let mut g = Grid::with_points(points.iter());
        g.cells.insert(Point { x: 0, y: 0 }, 0);

        g.tick();
        assert!(!g.cells.contains_key(&Point { x: 1, y: 1 }));
    }

    #[test]
    fn grid_tick_will_make_a_cell_alive_if_it_has_3_neighbors() {
        let points = [Point { x: 0, y: 1 },
                      Point { x: -1, y: 0 },
                      Point { x: 1, y: 0 }];
        let mut g = Grid::with_points(points.iter());

        g.tick();

        let point = Point { x: 0, y: 0 };
        assert!(g.cells.contains_key(&point));
    }

    #[test]
    fn grid_tick_will_preserve_cells_with_2_neighbors() {
        let points = [Point { x: 5, y: 1 },
                      Point { x: 5, y: 2 },
                      Point { x: 5, y: 3 }];
        let mut g = Grid::with_points(points.iter());
        g.tick();

        assert!(g.cells.contains_key(&Point { x: 5, y: 2 }));
    }

    #[test]
    fn grid_tick_will_preserve_cells_with_3_neighbors() {
        let points = [Point { x: 5, y: 1 },
                      Point { x: 5, y: 2 },
                      Point { x: 6, y: 2 },
                      Point { x: 5, y: 3 }];
        let mut g = Grid::with_points(points.iter());
        g.tick();

        let point = Point { x: 5, y: 2 };
        match g.cells.get(&point) {
            Some(generation) => assert_eq!(0, generation.clone()),
            None => panic!("Point not found"),
        }
    }

    #[test]
    fn grid_tick_advances_the_generation() {
        let mut g = Grid::empty();
        assert_eq!(0, g.generation);
        g.tick();
        assert_eq!(1, g.generation);
    }

    #[test]
    fn grid_tick_new_cells_are_stored_with_their_birth_generation() {
        let points = [Point { x: 0, y: 1 },
                      Point { x: -1, y: 0 },
                      Point { x: 1, y: 0 }];
        let mut g = Grid::with_points(points.iter());

        match g.cells.get(&points[0]) {
            Some(generation) => assert_eq!(0, generation.clone()),
            None => panic!("Point not found"),
        }

        g.tick();

        let point = Point { x: 0, y: 0 };
        match g.cells.get(&point) {
            Some(generation) => assert_eq!(1, generation.clone()),
            None => panic!("Point not found"),
        }

    }

    #[test]
    fn age_of_point_returns_none_if_point_is_dead() {
        let g = Grid::empty();
        assert_eq!(None, g.age_of_point(&Point { x: 0, y: 0 }));
    }

    #[test]
    fn age_of_point_returns_some_age_if_point_is_alive() {
        let points = [Point { x: 0, y: 1 }];
        let g = Grid::with_points(points.iter());
        assert_eq!(Some(0), g.age_of_point(&points[0]));
    }
}
