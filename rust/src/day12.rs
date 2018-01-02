use common;
use std::collections::HashMap;
use std::collections::HashSet;

struct Village {
    programs: HashMap<u32, HashSet<u32>>,
}

impl Village {
    fn new() -> Self {
        Village { programs: HashMap::new() }
    }

    fn connect(&mut self, prog: u32, other: u32) {
        self.insert(prog, other);
        self.insert(other, prog);
    }

    fn get(&self, prog: &u32) -> &HashSet<u32> {
        self.programs.get(prog).unwrap()
    }

    fn insert(&mut self, prog: u32, other: u32) {
        let set: &mut HashSet<u32> = self.programs.entry(prog).or_insert(HashSet::new());
        set.insert(other);
    }

    fn connected_to(&self, start: u32) -> Vec<u32> {
        let mut searcher = Searcher::new(&self);
        searcher.find(start)
    }

    fn num_groups(&self) -> u32 {
        let mut visited: HashSet<u32> = HashSet::new();
        let mut results: HashSet<String> = HashSet::new();
        for &prog in self.programs.keys() {
            if !visited.contains(&prog) {
                let connected = self.connected_to(prog);
                visited.extend(connected.clone().into_iter());
                let nums: Vec<String> = connected.into_iter().map(|n| n.to_string()).collect();
                let connected = nums.join(",");
                results.insert(connected);
            }
        }

        results.len() as u32
    }
}

struct Searcher<'a> {
    village: &'a Village,
    found: HashSet<u32>,
    checked: HashSet<u32>,
}

impl<'a> Searcher<'a> {
    fn new(village: &'a Village) -> Self {
        Self { village, found: HashSet::new(), checked: HashSet::new() }
    }

    fn find(&mut self, target: u32) -> Vec<u32> {
        if !self.checked.contains(&target) {
            self.checked.insert(target);
            let set = self.village.get(&target);
            for item in set.iter() {
                self.found.insert(*item);
                self.find(*item);
            }
        }

        let mut result: Vec<u32> = self.found.iter().map(|v| *v).collect();
        result.sort();
        result
    }
}

pub fn run(_args: &[String]) {
    let input = common::get_input("./inputs/12.txt").expect("expected input 12.txt");

    let mut village = Village::new();
    for line in input.lines() {
        let idx = line.find(" <-> ").unwrap();
        let prog: u32 = (&line[..idx]).parse().unwrap();
        let others: Vec<u32> = (&line[idx+5..]).split(", ").map(|s| s.parse().unwrap()).collect();
        for o in others {
            village.connect(prog, o);
        }
    }
    let connected = village.connected_to(0);
    // Part 1
    println!("Part 1: {} programs in program 0's group", connected.len());
    // Part 2
    println!("Part 2: {} groups", village.num_groups());
}

#[test]
fn test_village() {
    let mut v = Village::new();
    v.connect(1, 2);
    v.connect(1, 3);
    v.connect(2, 4);
    v.connect(5, 6);
    assert_eq!(v.connected_to(1), vec![1, 2, 3, 4]);
    assert_eq!(v.connected_to(5), vec![5, 6]);
    assert_eq!(v.connected_to(6), vec![5, 6]);
    assert_eq!(v.num_groups(), 2);
}
