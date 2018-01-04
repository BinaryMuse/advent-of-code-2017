use common;
use day10::KnotHasher;
use std::collections::HashSet;

pub fn run(_args: &[String]) {
    let input = common::get_input("./inputs/14.txt").expect("expected input 14.txt");
    let bins: Vec<Vec<bool>> = (0..128).map(|i| {
        let s = format!("{}-{}", input, i);
        let hash = KnotHasher::hash(&s);
        let bin = hex_to_bin(&hash);
        bin.chars().map(|c| c == '1').collect()
    }).collect();
    let used_count = bins.iter().map(|bin| bin.iter().filter(|v| **v == true).count() as i32).sum::<i32>();
    println!("Part 1: {} used spaces", used_count);

    // Feeling lazy
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut groups = 0;
    for i in 0i32..128 {
        for j in 0i32..128 {
            let pair = (i, j);
            let used = bins[i as usize][j as usize];
            if !visited.contains(&pair) {
                visited.insert(pair.clone());
                if used {
                    groups += 1;
                    traverse(&bins, &mut visited, pair);
                }
            }
        }
    }
    println!("Part 2: {} groups", groups);
}

fn traverse(bins: &Vec<Vec<bool>>, visited: &mut HashSet<(i32, i32)>, pair: (i32, i32)) {
    let (x, y) = pair;
    let neighbors = [(x+1, y), (x-1, y), (x, y+1), (x, y-1)];
    let valid = neighbors.into_iter().filter(|p| {
        is_valid_coord(p)
    }).collect::<Vec<_>>();
    for pair2 in valid {
        let &(x2, y2) = pair2;
        let used = bins[x2 as usize][y2 as usize];
        if !visited.contains(&pair2) {
            visited.insert(pair2.clone());
            if used {
                traverse(bins, visited, *pair2);
            }
        }
    }

    // let mut v: Vec<(usize, usize)> = Vec::with_capacity(8);
    //
    // for dx in -1i64..2 {
    //     for dy in -1i64..2 {
    //         let cell = ((x as i64 + dx) as usize, (y as i64 + dy) as usize);
    //         if cell != pair && is_valid_coord(cell) { // we're not our own neighbor
    //             v.push(cell);
    //         }
    //     }
    // }
}

fn is_valid_coord(pair: &(i32, i32)) -> bool {
    let (x, y) = *pair;
    x >= 0 && y >= 0 && x <= 127 && y <= 127
}

fn hex_to_bin(hex: &str) -> String {
    hex.chars().map(|c| {
        let n = u64::from_str_radix(&c.to_string(), 16).unwrap();
        format!("{:04b}", n)
    }).collect::<Vec<_>>().join("")
}

#[test]
fn test_hex_to_bin() {
    assert_eq!(hex_to_bin("a0c2017"), "1010000011000010000000010111");
}
