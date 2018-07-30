use common;
use coords::Coord;
use std::fmt;
// use std::str::FromStr;

pub fn run(_args: &[String]) {
    let _input = common::get_input("./inputs/21.txt").expect("expected input 21.txt");
    // let rulebook: HashMap<String, String> = parse_rules(&input);
}


enum Axis {
    VERTICAL, HORIZONTAL
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct PixBuf {
    size: usize,
    pixels: Vec<bool>,
}

impl PixBuf {
    fn from_str(string: &str) -> Self {
        let parts = string.trim().split("/").collect::<Vec<_>>();
        let size = parts.len();
        assert!(size > 0);
        let pixels = parts.join("").chars().map(|ch| {
            match ch {
                '#' => true,
                '.' => false,
                _   => panic!("Invalid pattern character")
            }
        }).collect::<Vec<_>>();
        PixBuf { size, pixels }
    }

    fn with_size(size: usize) -> Self {
        let pixels = vec![false; size.pow(2)];
        PixBuf { size, pixels }
    }

    fn stitch(pixbufs: &Vec<PixBuf>) -> Self {
        let len = pixbufs.len();
        assert!(len > 0);
        let size = pixbufs[0].size;
        assert!(pixbufs.iter().all(|pb| pb.size == size));
        let num_bufs_per_row = (len as f64).sqrt() as usize;
        let size_total = num_bufs_per_row * size;

        let mut buf = PixBuf::with_size(size_total);

        for y in 0..size {
            for x in 0..size {
                let start = Coord(x * size, y * size);
                let idx = y * num_bufs_per_row + x;
                let copy_from = &pixbufs[idx];
                buf.copy_from(copy_from, start);
            }
        }

        buf
    }

    fn index_for_coord(&self, coord: Coord) -> usize {
        let Coord(x, y) = coord;
        y * self.size + x
    }

    fn get_pixel(&self, coord: Coord) -> bool {
        self.pixels[self.index_for_coord(coord)]
    }

    fn set_pixel(&mut self, coord: Coord, value: bool) -> &mut Self {
        let index = self.index_for_coord(coord);
        self.pixels[index] = value;
        self
    }

    fn swap_pixels(&mut self, coord1: Coord, coord2: Coord) -> &mut Self {
        let idx1 = self.index_for_coord(coord1);
        let idx2 = self.index_for_coord(coord2);
        self.pixels.swap(idx1, idx2);
        self
    }

    fn copy_from(&mut self, other: &PixBuf, offset: Coord) -> &mut Self {
        assert!(Coord(self.size, self.size) >= Coord(other.size, other.size) + offset);

        for x in 0..other.size {
            for y in 0..other.size {
                let new_x = x + offset.0;
                let new_y = y + offset.1;
                self.set_pixel(Coord(new_x, new_y), other.get_pixel(Coord(x, y)));
            }
        }

        self
    }

    fn split_into_sized(&self, size: usize) -> Vec<PixBuf> {
        assert!(self.size % size == 0);
        let half = self.size / size;
        let mut result = Vec::with_capacity(half);
        // Iterate columns-per-row first
        for y in 0..half {
            for x in 0..half {
                let start = Coord(x * size, y * size);
                let extracted = self.extract_region(start, size);
                result.push(extracted);
            }
        }
        result
    }

    fn extract_region(&self, start: Coord, size: usize) -> PixBuf {
        let mut buf = PixBuf::with_size(size);
        for x in 0..size {
            for y in 0..size {
                let value = self.get_pixel(Coord(x, y) + start);
                buf.set_pixel(Coord(x, y), value);
            }
        }

        buf
    }

    fn flip(&mut self, across_axis: Axis) -> &mut Self {
        let mid = self.size / 2;

        for y in 0..self.size {
            for x in 0..self.size {
                let matches = match across_axis {
                    Axis::VERTICAL   => y < mid,
                    Axis::HORIZONTAL => x < mid
                };

                if !matches {
                    continue;
                }

                let opposite_coord = match across_axis {
                    Axis::VERTICAL   => Coord(x, self.size - y - 1),
                    Axis::HORIZONTAL => Coord(self.size - x - 1, y)
                };

                self.swap_pixels(Coord(x, y), opposite_coord);
            }
        }

        self
    }

    fn rotate(&mut self) -> &mut Self {
        // Transpose...
        for i in 0..(self.size - 1) {
            for j in (i+1)..(self.size) {
                let coord1 = Coord(i, j);
                let coord2 = Coord(j, i);
                self.swap_pixels(coord1, coord2);
            }
        }

        // ...then flip to get a 90-degree rotation
        self.flip(Axis::HORIZONTAL)
    }

    fn to_string(&self) -> String {
        self.pixels.chunks(self.size).map(|chunk| {
            chunk.iter().map(|b| if *b { "#" } else { "." }).collect::<String>()
        }).collect::<Vec<_>>().join("/")
    }
}

impl fmt::Debug for PixBuf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PixBuf<{},\"{}\">", self.size, self.to_string())
    }
}

#[test]
fn test_pixbuf_flipping() {
    let pix1 = PixBuf::from_str("#.#/.#./##.");
    let flipped_vert = PixBuf::from_str("##./.#./#.#");
    let flipped_horiz = PixBuf::from_str("#.#/.#./.##");
    assert_eq!(pix1.clone().flip(Axis::VERTICAL), &flipped_vert);
    assert_eq!(pix1.clone().flip(Axis::HORIZONTAL), &flipped_horiz);
}

#[test]
fn test_pixbuf_rotation() {
    let mut pix1 = PixBuf::from_str("#.#/..#/.#.");
    let rotate1 = PixBuf::from_str("..#/#../.##");
    let rotate2 = PixBuf::from_str(".#./#../#.#");
    let rotate3 = PixBuf::from_str("##./..#/#..");
    let rotate4 = pix1.clone();
    assert_eq!(pix1.rotate(), &rotate1);
    assert_eq!(pix1.rotate(), &rotate2);
    assert_eq!(pix1.rotate(), &rotate3);
    assert_eq!(pix1.rotate(), &rotate4);
}

#[test]
fn test_pixbuf_apply() {
    let mut pix1 = PixBuf::from_str("..../..../..../....");
    let pix2 = PixBuf::from_str("##/##");
    pix1.copy_from(&pix2, Coord(1, 1));
    assert_eq!(pix1.to_string(), "..../.##./.##./....");
}

#[test]
fn test_pixbuf_split() {
    let pix1 = PixBuf::from_str(".##./#..#/..##/##..");
    let split = pix1.split_into_sized(2);
    let strings = split.iter().map(|pb| pb.to_string()).collect::<Vec<_>>();
    assert_eq!(strings, vec![
        ".#/#.", "#./.#", "../##", "##/.."
    ]);
}

#[test]
fn test_pixbuf_stitch() {
    let pixbufs = vec![".#/#.", "#./.#", "../##", "##/.."].into_iter().map(PixBuf::from_str).collect::<Vec<_>>();
    assert_eq!(PixBuf::from_str(".##./#..#/..##/##.."), PixBuf::stitch(&pixbufs));
}


#[derive(Eq, PartialEq, Hash, Clone)]
struct Rule {
    pattern: PixBuf,
    replacement: PixBuf,
}

impl Rule {
    fn from_str(string: &str) -> Self {
        let parts = string.split(" => ").collect::<Vec<_>>();
        let pattern = PixBuf::from_str(parts[0]);
        let replacement = PixBuf::from_str(parts[1]);
        Rule { pattern, replacement }
    }

    fn new(pattern: PixBuf, replacement: PixBuf) -> Self {
        Self { pattern, replacement }
    }

    fn get_replacement(&self) -> &PixBuf {
        &self.replacement
    }

    fn matches(&self, check: &PixBuf) -> bool {
        // PixBufs of different sizes can never match
        if self.pattern.size != check.size {
            return false;
        }

        if check == &self.pattern {
            return true;
        }

        let mut clone = check.clone();
        for _i in 0..4 {
            clone.rotate();
            if clone == self.pattern {
                return true;
            }
            clone.flip(Axis::HORIZONTAL);
            if clone == self.pattern {
                return true;
            }
            clone.flip(Axis::VERTICAL);
            if clone == self.pattern {
                return true;
            }
            clone.flip(Axis::VERTICAL);
            clone.flip(Axis::HORIZONTAL);
        }

        false
    }
}

#[test]
fn test_rule_matching() {
    let pattern = PixBuf::from_str(".#./..#/###");
    let replacement = PixBuf::from_str("#..#/..../..../#..#");
    let rule = Rule::new(pattern.clone(), replacement);

    for i in 0..5 {
        let mut to_match = pattern.clone();
        for _j in 0..i {
            println!("Rotating...");
            to_match.rotate();
        };
        println!("Rotated {} times", i);
        assert!(rule.matches(&to_match));
        assert!(rule.matches(to_match.clone().flip(Axis::VERTICAL)));
        assert!(rule.matches(to_match.clone().flip(Axis::HORIZONTAL)));
        assert!(rule.matches(to_match.clone().flip(Axis::HORIZONTAL).flip(Axis::VERTICAL)));
    };
}


struct Rulebook {
    rules: Vec<Rule>
}

impl Rulebook {
    fn from_str(string: &str) -> Self {
        let rules = string.trim().lines().map(Rule::from_str).collect::<Vec<_>>();
        Rulebook { rules }
    }

    fn get_replacement(&self, search: &PixBuf) -> Option<&PixBuf> {
        let matching = &self.rules.iter().find(|rule| rule.matches(search));
        match matching {
            Some(ref rule) => Some(rule.get_replacement()),
            None           => None
        }
    }
}

#[test]
fn test_parse_rulebook() {
    let rulebook_text = "../.# => ##./#../...\n.#./..#/### => #..#/..../..../#..#";
    let rulebook = Rulebook::from_str(rulebook_text);
    let search = PixBuf::from_str(".#./..#/###");
    let replacement = PixBuf::from_str("#..#/..../..../#..#");
    assert_eq!(rulebook.get_replacement(&search), Some(&replacement));
}
