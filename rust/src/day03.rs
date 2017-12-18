use common;
use std::collections::HashMap;

/// An `(x, y)` coordinate pair.
type Coord = (i32, i32);

/// A Ulam spiral is a square spiral starting at the origin
/// and proceeding in a counter-clockwise direction:
///
///           -3  -2  -1  0   1   2   3
///         /--------------------------
///     3  | 37  36  35  34  33  32  31
///     2  | 38  17  16  15  14  13  30
///     1  | 39  18   5   4   3  12  29
///     0  | 40  19   6   1   2  11  28
///     -1 | 41  20   7   8   9  10  27
///     -2 | 42  21  22  23  24  25  26
///     -3 | 43  44  45  46  47  48  49
///
/// This implementation of the Ulam spiral allows one to store
/// values of type `T` at various `(x, y)` coordinates (and fetch the values later),
/// find all the neighbors of any given coordinate, and traverse the spiral
/// in the order described above.
struct Ulam<T: Copy> {
    explored: HashMap<Coord, T>,
}

impl<T: Copy> Ulam<T> {
    /// Creates a new empty Ulam spiral.
    fn new() -> Self {
        Ulam { explored: HashMap::new() }
    }

    fn set(&mut self, coord: Coord, value: T) {
        self.explored.insert(coord, value);
    }

    fn get(&self, coord: Coord) -> Option<&T> {
        self.explored.get(&coord)
    }

    fn neighbors(&self, coord: Coord) -> Vec<Coord> {
        let (x, y) = coord;
        let mut v = Vec::with_capacity(8);

        for dx in -1..2 {
            for dy in -1..2 {
                let cell = (x + dx, y + dy);
                if cell != coord { // we're not our own neighbor
                    v.push(cell);
                }
            }
        }

        v
    }

    fn neighbors_values(&self, coord: Coord) -> Vec<Option<&T>> {
        self.neighbors(coord).iter().map(|coord| self.explored.get(coord)).collect()
    }

    fn iter(&self) -> UlamIterator {
        UlamIterator::new()
    }
}


/// A direction that we're currently traversing in the grid.
enum UlamIterDirection {
    UP, DOWN, LEFT, RIGHT
}

impl UlamIterDirection {
    /// Returns the direction we'd prefer to be moving to next,
    /// assuming that the cell in that direction isn't already taken.
    /// For example, when moving to the right, we want to turn so we're
    /// moving up — but only if the space above us is empty.
    fn preferred_next_direction(&self) -> UlamIterDirection {
        use self::UlamIterDirection::*;
        match self {
            &UP    => LEFT,
            &LEFT  => DOWN,
            &DOWN  => RIGHT,
            &RIGHT => UP
        }
    }

    /// Returns the (x, y) delta to move in the direction.
    fn delta(&self) -> (i32, i32) {
        use self::UlamIterDirection::*;
        match self {
            &UP    => ( 0,  1),
            &DOWN  => ( 0, -1),
            &LEFT  => (-1,  0),
            &RIGHT => ( 1,  0)
        }

    }
}

/// An iterator that produces `(x, y)` tuples in the order that
/// one traverses a Ulam spiral. Owns its own `Ulam` instance to
/// keep track of which cells have been visited.
struct UlamIterator {
    ulam: Ulam<bool>,
    current: Option<Coord>,
    direction: Option<UlamIterDirection>,
}

impl UlamIterator {
    /// Create anew `UlamIterator` starting at the origin.
    fn new() -> Self {
        UlamIterator {
            ulam: Ulam::new(),
            current: None,
            direction: None
        }
    }
}

impl Iterator for UlamIterator {
    type Item = Coord;

    /// Yields the next `(x, y)` coordinate pair in the Ulam spiral, following
    /// the direction of the spiral. This method never returns `None` (there can
    /// always be a next cell); be sure to constrain consumers appropriately.
    fn next(&mut self) -> Option<Coord> {
        match self.current {
            // If we haven't yet started traversing the spiral,
            // then the first element is `(0, 0)`.
            None => {
                let new_cell = (0, 0);
                self.current = Some(new_cell);
                self.ulam.set(new_cell, true);
                Some(new_cell)
            }
            // If we've already started traversing the spiral...
            Some((x, y)) => {
                // ...then determine the next direction we should go (e.g. if we're
                // moving to the right, then we want to go up next if we can).
                // If we haven't started moving yet, we move to the right.
                let preferred_next_direction = match self.direction {
                    Some(ref dir) => dir.preferred_next_direction(),
                    None          => UlamIterDirection::RIGHT
                };
                // Determine the next cell we'd like to move to.
                let (dx, dy) = preferred_next_direction.delta();
                let check_cell = (x + dx, y + dy);
                match self.ulam.get(check_cell) {
                    // If the cell we want to move to is taken, we simply
                    // keep going our current direction.
                    Some(_) => {
                        let (dx, dy) = self.direction.as_ref().unwrap().delta();
                        let next_cell = (x + dx, y + dy);
                        self.current = Some(next_cell);
                        self.ulam.set(next_cell, true);
                        Some(next_cell)
                    },
                    // The cell we want to move to is free! Happy day!
                    // Let's move to that one, and set our direction so we continue
                    // moving in that direction in the next iteration.
                    None => {
                        self.current = Some(check_cell);
                        self.ulam.set(check_cell, true);
                        self.direction = Some(preferred_next_direction);
                        Some(check_cell)
                    }
                }
            }
        }
    }
}

#[test]
fn test_grid_set_get() {
    let mut g: Ulam<u32> = Ulam::new();
    g.set((0, 0), 42);
    assert_eq!(g.get((0, 0)), Some(&42));
    assert_eq!(g.get((0, 1)), None);
}

#[test]
fn test_grid_neighbors_values() {
    let mut g: Ulam<u32> = Ulam::new();
    assert_eq!(g.neighbors_values((0, 0)), vec![None, None, None, None, None, None, None, None]);

    g.set((-1,  1), 1);
    g.set(( 0,  1), 2);
    g.set(( 1,  1), 3);
    g.set((-1,  0), 4);
    g.set(( 0,  0), 5);
    g.set(( 1,  0), 6);
    g.set((-1, -1), 7);
    g.set(( 0, -1), 8);
    g.set(( 1, -1), 9);

    let total = g.neighbors_values((0, 0))
        .iter()
        .map(|opt| opt.unwrap())
        .fold(0, |acc, val| acc + val);
    assert_eq!(total, 1 +2 + 3 + 4 + 6 + 7 + 8 + 9);
}

#[test]
fn test_grid_iteration() {
    let g: Ulam<u32> = Ulam::new();
    let mut v: Vec<Coord> = Vec::new();
    for cell in g.iter().take(26) {
        v.push(cell);
    }

    assert_eq!(v, vec![
        (0, 0), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1), // 1 to 9
        (2, -1), (2, 0), (2, 1), (2, 2), (1, 2), (0, 2), (-1, 2), (-2, 2), (-2, 1), // 10 to 18
        (-2, 0), (-2, -1), (-2, -2), (-1, -2), (0, -2), (1, -2), (2, -2), (3, -2) // 19 to 26
    ]);
}

fn distance_for_square(n: u64) -> i32 {
    let g: Ulam<u64> = Ulam::new();
    for (i, cell) in g.iter().enumerate() {
        if (i as u64 + 1) == n {
            let (x, y) = cell;
            return x.abs() + y.abs();
        }
    }

    0
}

#[test]
fn test_distance_for_square() {
    assert_eq!(distance_for_square(1), 0);
    assert_eq!(distance_for_square(12), 3);
    assert_eq!(distance_for_square(23), 2);
    assert_eq!(distance_for_square(1024), 31);
}

#[test]
fn test_day03() {
    // Part 1 - takes some time to calculate in debug builds
    // assert_eq!(distance_for_square(289326), 419);

    // Part 2
    assert_eq!(part2(289326), 295229);
}

pub fn run(_args: &[String]) {
    let input = common::get_input("./inputs/03.txt").expect("expected input 03.txt");
    let num: u64 = input.parse().expect("couldn't parse input as a number");

    println!("Part 1: {}", distance_for_square(num));
    println!("Part 2: {}", part2(num));
}

fn part2(check: u64) -> u64 {
    //  Create a new Ulam spiral
    let mut g: Ulam<u64> = Ulam::new();
    // Prime the first cell
    g.set((0, 0), 1);

    // Skipping the first cell ...
    for cell in g.iter().skip(1) {
        let sum = {
            // ... find all the values of the neighbors ...
            let values = g.neighbors_values(cell);
            // ... and sum the ones that exist (the ones that don't default to 0)
            values.iter().map(|opt| opt.unwrap_or(&0)).fold(0, |acc, val| acc + val)
        };
        // Set the sum for the current cell
        g.set(cell, sum);
        // Check to see if we found our magic number
        if sum > check {
            return sum
        }
    }

    0
}
