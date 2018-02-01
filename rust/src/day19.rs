use common;
use std::collections::HashMap;

pub fn run(_args: &[String]) {
    let input = common::get_input("./inputs/19.txt").expect("expected input 19.txt");
    let maze = Maze::from_map(&input);
    let treasure = maze.find_treasure().iter().collect::<String>();
    println!("Part 1: {}", treasure);

    println!("Part 2: {}", maze.iter().count());
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction { N, S, E, W }

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            &Direction::N => Direction::S,
            &Direction::S => Direction::N,
            &Direction::E => Direction::W,
            &Direction::W => Direction::E,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Cell(i64, i64);

/// Maze coordinates start at (0, 0) for the upper-left.
/// `x` increases to the right, `y` increases downward.
impl Cell {
    pub fn neighbor(&self, dir: Direction) -> Self {
        use self::Direction::*;
        match dir {
            N => self.north(),
            S => self.south(),
            E => self.east(),
            W => self.west(),
        }
    }

    fn north(&self) -> Self {
        Cell(self.0, self.1 - 1)
    }

    fn south(&self) -> Self {
        Cell(self.0, self.1 + 1)
    }

    fn east(&self) -> Self {
        Cell(self.0 + 1, self.1)
    }

    fn west(&self) -> Self {
        Cell(self.0 - 1, self.1)
    }

    fn direction_from(&self, previous: Cell) -> Option<Direction> {
        let delta_x = previous.0 - self.0;
        let delta_y = previous.1 - self.1;

        match (delta_x, delta_y) {
            (0,  1) => Some(Direction::N),
            (0, -1) => Some(Direction::S),
            ( 1, 0) => Some(Direction::W),
            (-1, 0) => Some(Direction::E),
            _       => None,
        }
    }
}

#[test]
fn test_cell_neighbors() {
    let cell = Cell(4, -3);
    assert_eq!(cell.north(), Cell(4, -4));
    assert_eq!(cell.south(), Cell(4, -2));
    assert_eq!(cell.east(), Cell(5, -3));
    assert_eq!(cell.west(), Cell(3, -3));
}

#[test]
fn test_cell_direction() {
    let cell = Cell(5, 5);
    assert_eq!(cell.direction_from(Cell(4, 5)), Some(Direction::E));
    assert_eq!(cell.direction_from(Cell(6, 5)), Some(Direction::W));
    assert_eq!(cell.direction_from(Cell(5, 4)), Some(Direction::S));
    assert_eq!(cell.direction_from(Cell(5, 6)), Some(Direction::N));
}

#[derive(Debug, PartialEq, Eq)]
enum RoomType { NS, EW, Corner, Letter(char), Empty }

#[derive(Debug, PartialEq, Eq)]
struct Room {
    kind: RoomType,
}

impl Room {
    fn new(kind: RoomType) -> Self {
        Self { kind }
    }
}

struct Maze {
    rooms: HashMap<Cell, Room>,
}

impl Maze {
    fn from_map(map: &str) -> Self {
        let mut rooms = HashMap::new();

        for (line_idx, line) in map.trim_right_matches("\n").lines().enumerate() {
            for (char_idx, ch) in line.chars().enumerate() {
                let cell = Cell(char_idx as i64, line_idx as i64);
                let room = match ch {
                    '|' => RoomType::NS,
                    '-' => RoomType::EW,
                    '+' => RoomType::Corner,
                    ' ' => RoomType::Empty,
                    _   => RoomType::Letter(ch),
                };
                rooms.insert(cell, Room::new(room));
            }
        }

        Self { rooms }
    }

    fn get_start(&self) -> Option<Cell> {
        let start_pair = self.rooms.iter().find(|&(cell, room)| {
            room.kind == RoomType::NS && cell.1 == 0
        });

        start_pair.map(|pair| *pair.0)
    }

    fn get_room(&self, cell: &Cell) -> Option<&Room> {
        self.rooms.get(cell)
    }

    fn iter(&self) -> MazeIter {
        MazeIter::new(self)
    }

    fn find_treasure(&self) -> Vec<char> {
        let mut treasure = vec![];
        for cell in self.iter() {
            let room = self.get_room(&cell).expect("Expected iter'd room to exist");
            if let RoomType::Letter(ch) = room.kind {
                treasure.push(ch);
            }
        }

        treasure
    }
}

struct MazeIter<'a> {
    maze: &'a Maze,
    last_cell: Option<Cell>,
    direction: Direction,
}

impl<'a> MazeIter<'a> {
    fn new(maze: &'a Maze) -> Self {
        Self { maze, last_cell: None, direction: Direction::S }
    }

    fn cell_after(&self, cell: Cell) -> Option<Cell> {
        use self::RoomType::*;

        let room = self.maze.get_room(&cell).expect("No room found for visited cell!");
        match room.kind {
            NS | EW | Letter(_) => {
                let next = cell.neighbor(self.direction);
                if let Some(room) = self.maze.get_room(&next) {
                    if room.kind == RoomType::Empty {
                        None
                    } else {
                        Some(next)
                    }
                } else {
                    None
                }
            },
            Corner => {
                let candidate_neighbors = vec![cell.north(), cell.south(), cell.east(), cell.west()];
                let remaining = candidate_neighbors.iter().filter(|&&c| {
                    if let Some(room) = self.maze.get_room(&c) {
                        let previous = cell.neighbor(self.direction.opposite());
                        c != previous && room.kind != RoomType::Empty
                    } else {
                        false
                    }
                }).collect::<Vec<_>>();
                assert!(remaining.len() == 1);
                Some(*remaining[0])
            },
            Empty => {
                panic!("Shouldn't be able to visit Empty room");
            },
        }
    }
}

impl<'a> Iterator for MazeIter<'a> {
    type Item = Cell;

    fn next(&mut self) -> Option<Self::Item> {
        let next_cell = match self.last_cell {
            Some(cell) => self.cell_after(cell),
            None       => self.maze.get_start(),
        };

        if next_cell.is_none() {
            return None
        }

        let next_direction = match self.last_cell {
            Some(cell) => next_cell.unwrap().direction_from(cell).unwrap(),
            None       => Direction::S,
        };

        self.last_cell = next_cell;
        self.direction = next_direction;

        next_cell
    }
}

#[test]
fn test_maze() {
    let input = "     |          \n     |  +--+    \n     A  |  C    \n F---|----E|--+ \n     |  |  |  D \n     +B-+  +--+ \n";
    let maze = Maze::from_map(input);
    assert_eq!(maze.get_start(), Some(Cell(5, 0)));

    let mut iter = maze.iter();
    assert_eq!(iter.next(), Some(Cell(5, 0)));
    assert_eq!(iter.next(), Some(Cell(5, 1)));
    assert_eq!(iter.next(), Some(Cell(5, 2)));
    assert_eq!(iter.next(), Some(Cell(5, 3)));
    assert_eq!(iter.next(), Some(Cell(5, 4)));
    assert_eq!(iter.next(), Some(Cell(5, 5)));
    assert_eq!(iter.next(), Some(Cell(6, 5)));

    assert_eq!(maze.find_treasure().iter().collect::<String>(), "ABCDEF");
}
