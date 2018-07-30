use std::ops::Add;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Coord(pub usize, pub usize);

impl Add<Self> for Coord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Coord(self.0 + other.0, self.1 + other.1)
    }
}

impl From<(usize, usize)> for Coord {
    fn from(tuple: (usize, usize)) -> Self {
        Coord(tuple.0, tuple.1)
    }
}

impl From<Coord> for (usize, usize) {
    fn from(coord: Coord) -> (usize, usize) {
        (coord.0, coord.1)
    }
}
