use common;

#[derive(Clone)]
struct Layer {
    depth: u32,
    range: u32,
}

impl Layer {
    fn new(depth: u32, range: u32) -> Self {
        Self { depth, range }
    }

    fn default() -> Self {
        Self::new(0, 0)
    }

    fn pos_at(&self, turn: i32) -> Option<u32> {
        if self.range <= 0 {
            return None;
        }

        let period = (self.range - 1) as i32;
        let base = turn / period;
        let r: i32     = if base % 2 == 0 { 0 } else { period };
        let delta: i32 = if base % 2 == 0 { 1 } else { -1 };
        let rem = turn % period;
        Some((r + rem * delta) as u32)
    }
}

#[test]
fn test_layer() {
    let l1 = Layer::new(0, 3);
    assert_eq!(l1.pos_at(0), Some(0));
    assert_eq!(l1.pos_at(1), Some(1));
    assert_eq!(l1.pos_at(2), Some(2));
    assert_eq!(l1.pos_at(3), Some(1));
    assert_eq!(l1.pos_at(4), Some(0));
    assert_eq!(l1.pos_at(5), Some(1));

    let l2 = Layer::new(3, 6);
    assert_eq!(l2.pos_at(0), Some(0));
    assert_eq!(l2.pos_at(1), Some(1));
    assert_eq!(l2.pos_at(2), Some(2));
    assert_eq!(l2.pos_at(3), Some(3));
    assert_eq!(l2.pos_at(4), Some(4));
    assert_eq!(l2.pos_at(5), Some(5));
}

#[derive(Clone)]
struct Firewall {
    layers: Vec<Layer>,
}

impl Firewall {
    fn parse(s: &str) -> Self {
        let mut layers = Vec::new();

        let mut current_layer = 0;
        for line in s.lines() {
            let parts = line.split(": ").collect::<Vec<_>>();
            let depth = parts[0].parse::<u32>().unwrap();
            let range = parts[1].parse::<u32>().unwrap();
            let layer = Layer::new(depth, range);

            for _ in current_layer..depth {
                layers.push(Layer::default())
            }
            layers.push(layer);
            current_layer = depth + 1;
        }

        Firewall { layers }
    }

    fn traverse(&self) -> Option<u32> {
        self.traverse_delayed(0)
    }

    fn traverse_delayed(&self, delay: u32) -> Option<u32> {
        let mut severity = None;
        for i in 0..self.layers.len() {
            let layer = &self.layers[i];
            if layer.pos_at(i as i32 + delay as i32) == Some(0) {
                let penalty = layer.range * layer.depth;
                severity = Some(severity.map_or(penalty, |sev| sev + penalty));
            }
        }

        severity
    }
}

#[test]
fn test_firewall() {
    let firewall = Firewall::parse("0: 3\n1: 2\n4: 4\n6: 4");
    assert_eq!(firewall.layers.len(), 7);
    assert_eq!(firewall.traverse(), Some(24));
}

fn find_safe_delay(firewall: &Firewall) -> Option<u32> {
    (0..).find(|&i| firewall.traverse_delayed(i as u32) == None)
}

#[test]
fn test_find_safe_delay() {
    let firewall = Firewall::parse("0: 3\n1: 2\n4: 4\n6: 4");
    assert_eq!(find_safe_delay(&firewall), Some(10));
}

pub fn run(_args: &[String]) {
    let input = common::get_input("./inputs/13.txt").expect("expected input 13.txt");

    let firewall = Firewall::parse(input.trim());
    println!("Part 1: Severity was {:?}", firewall.traverse());
    let delay = find_safe_delay(&firewall);
    println!("Part 2: Delay {:?} picoseconds to get through the firewall", delay);
}
