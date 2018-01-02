use common;
use std::ops::Add;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Copy)]
enum HexDirection {
    N, NE, SE, S, SW, NW
}

impl FromStr for HexDirection {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::HexDirection::*;
        match s.to_uppercase().as_ref() {
            "N"  => Ok(N),
            "NE" => Ok(NE),
            "SE" => Ok(SE),
            "S"  => Ok(S),
            "SW" => Ok(SW),
            "NW" => Ok(NW),
            _    => Err("couldn't parse direction".to_string()),
        }
    }
}

#[test]
fn test_hex_direction() {
    assert_eq!("n".parse::<HexDirection>(), Ok(HexDirection::N));
    assert_eq!("sw".parse::<HexDirection>(), Ok(HexDirection::SW));
    assert!("ok".parse::<HexDirection>().is_err());
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Hex {
    q: i32,
    r: i32,
    s: i32,
}

impl Hex {
    fn origin() -> Self {
        Self::new(0, 0, 0)
    }

    fn new(q: i32, r: i32, s: i32) -> Self {
        Self { q, r, s }
    }

    fn distance(&self) -> i32 {
        (self.q.abs() + self.r.abs() + self.s.abs()) / 2
    }
}

impl From<HexDirection> for Hex {
    fn from(dir: HexDirection) -> Self {
        use self::HexDirection::*;
        match dir {
            N  => Hex::new(0, 1, -1),
            NE => Hex::new(1, 0, -1),
            SE => Hex::new(1, -1, 0),
            S  => Hex::new(0, -1, 1),
            SW => Hex::new(-1, 0, 1),
            NW => Hex::new(-1, 1, 0),
        }
    }
}

impl Add<Hex> for Hex {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Hex {
            q: self.q + other.q,
            r: self.r + other.r,
            s: self.s + other.s,
        }
    }
}

impl Add<HexDirection> for Hex {
    type Output = Self;

    fn add(self, other: HexDirection) -> Self {
        let other_hex: Self = other.into();
        self + other_hex
    }
}

#[test]
fn test_hex() {
    let h1 = Hex::new(1, 3, -2);
    let h2 = Hex::new(3, -4, 3);
    assert_eq!(h1 + h2, Hex { q: 4, r: -1, s: 1 });
    assert_eq!(h1 + HexDirection::N, Hex { q: 1, r: 4, s: -3 });
}

pub fn run(_args: &[String]) {
    let input = common::get_input("./inputs/11.txt").expect("expected input 11.txt");

    let directions: Vec<HexDirection> = input.trim().split(",").map(|s| s.parse().unwrap()).collect();

    // Part 1
    {
        let target = directions.iter().fold(Hex::origin(), |acc, &next| acc + next);
        println!("Part 1: The child process is at {:?}, which is {} units away", target, target.distance());
    }

    // Part 2
    {
        let mut location = Hex::origin();
        let mut max_distance = 0;
        for dir in directions {
            location = location + dir;
            let distance = location.distance();
            if distance > max_distance {
                max_distance = distance;
            }
        }
        println!("Part 2: The furthest the child got was {} units away", max_distance);
    }
}
