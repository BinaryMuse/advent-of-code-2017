use common;
use std::fmt;
// use std::str::FromStr;

pub fn run(_args: &[String]) {
    let _input = common::get_input("./inputs/21.txt").expect("expected input 21.txt");
    // let rulebook: HashMap<String, String> = parse_rules(&input);
}

// fn parse_rules(rules: &str) -> HashMap<String, String> {
//     return HashMap::new();
// }

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

    fn flip(&mut self, across_axis: Axis) -> &Self {
        let mut swaps: Vec<(usize, usize)> = vec![];
        for (idx, _value) in self.pixels.iter().enumerate() {
            let row = idx / self.size;
            let col = idx % self.size;
            let mid = self.size / 2;

            let opposite_row = self.size - row - 1;
            let opposite_col = self.size - col - 1;

            let matches = match across_axis {
                Axis::VERTICAL   => row < mid,
                Axis::HORIZONTAL => col < mid
            };

            if !matches {
                continue;
            }

            let opposite_index = match across_axis {
                Axis::VERTICAL   => opposite_row * self.size + col,
                Axis::HORIZONTAL => row * self.size + opposite_col
            };

            swaps.push((idx, opposite_index));
        };

        for (old, new) in swaps {
            self.pixels.swap(old, new);
        }

        self
    }

    fn rotate(&mut self) -> &Self {
        for i in 0..(self.size - 1) {
            for j in (i+1)..(self.size) {
                let old_index = i * self.size + j;
                let new_index = j * self.size + i;
                self.pixels.swap(old_index, new_index);
            }
        }

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

struct Rule {
    pattern: PixBuf,
    replacement: PixBuf,
}

impl Rule {
    fn new(pattern: PixBuf, replacement: PixBuf) -> Self {
        Self { pattern, replacement }
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
        clone.flip(Axis::VERTICAL);
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

        false
    }
}

#[test]
fn test_rule_matching() {
    let pattern = PixBuf::from_str(".#./..#/###");
    let replacement = PixBuf::from_str("#..#/..../..../#..#");
    let rule = Rule::new(pattern, replacement);

    assert!(rule.matches(&PixBuf::from_str(".#./..#/###")));
    assert!(rule.matches(&PixBuf::from_str(".#./#../###")));
}

// #[test]
// fn test_parse_rules() {
//     let rulebook_text = "../.# => ##./#../...\n.#./..#/### => #..#/..../..../#..#";
//     let rulebook = parse_rules(rulebook_text);
//     assert_eq!(rulebook.get(&String::from_str("../.#").unwrap()), Some(&String::from_str("##./#../...").unwrap()));
// }
