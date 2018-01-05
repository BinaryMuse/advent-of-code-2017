struct FactorGenerator {
    factor: u64,
    div: u64,
    prev: u64,
}

impl FactorGenerator {
    fn new(factor: u64, start: u64, div: u64) -> Self {
        FactorGenerator { factor, div, prev: start }
    }

    fn only_div(self, div: u64) -> DivFactorGenerator {
        DivFactorGenerator::new(self, div)
    }
}

impl Iterator for FactorGenerator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let mult = self.prev * self.factor;
        let rem = mult % self.div;
        self.prev = rem;
        Some(rem)
    }
}

struct DivFactorGenerator {
    gen: FactorGenerator,
    check: u64,
}

impl DivFactorGenerator {
    fn new(gen: FactorGenerator, check: u64) -> Self {
        DivFactorGenerator { gen, check }
    }
}

impl Iterator for DivFactorGenerator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(n) = self.gen.next() {
            if n % self.check == 0 {
                return Some(n);
            }
        }

        None
    }
}

pub fn run(_args: &[String]) {
    // Hard-coding input
    let gen_a_start = 277;
    let gen_a_factor = 16807;
    let gen_b_start = 349;
    let gen_b_factor = 48271;
    let div = 2147483647;

    {
        let gen_a = FactorGenerator::new(gen_a_factor, gen_a_start, div);
        let gen_b = FactorGenerator::new(gen_b_factor, gen_b_start, div);

        // This is sloooowwww
        let judged = gen_a.zip(gen_b).take(40_000_000).filter(|pair| {
            let &(a, b) = pair;
            let a_bin: String = format!("{:016b}", a).chars().rev().take(16).collect();
            let b_bin: String = format!("{:016b}", b).chars().rev().take(16).collect();
            a_bin == b_bin
        }).count();
        println!("Part 1: {} matches", judged);
    }

    {
        let gen_a = FactorGenerator::new(gen_a_factor, gen_a_start, div).only_div(4);
        let gen_b = FactorGenerator::new(gen_b_factor, gen_b_start, div).only_div(8);

        let judged = gen_a.zip(gen_b).take(5_000_000).filter(|pair| {
            let &(a, b) = pair;
            let a_bin: String = format!("{:016b}", a).chars().rev().take(16).collect();
            let b_bin: String = format!("{:016b}", b).chars().rev().take(16).collect();
            a_bin == b_bin
        }).count();
        println!("Part 2: {} matches", judged);
    }
}

#[test]
fn test_gens() {
    let a = FactorGenerator::new(16807, 65, 2147483647);
    let b = FactorGenerator::new(48271, 8921, 2147483647);

    assert_eq!(a.take(5).collect::<Vec<_>>(), vec![
        1092455, 1181022009, 245556042, 1744312007, 1352636452
    ]);
    assert_eq!(b.take(5).collect::<Vec<_>>(), vec![
        430625591, 1233683848, 1431495498, 137874439, 285222916
    ]);
}

#[test]
fn test_gens_only_div() {
    let a = FactorGenerator::new(16807, 65, 2147483647).only_div(4);
    let b = FactorGenerator::new(48271, 8921, 2147483647).only_div(8);

    assert_eq!(a.take(5).collect::<Vec<_>>(), vec![
        1352636452, 1992081072, 530830436, 1980017072, 740335192
    ]);
    assert_eq!(b.take(5).collect::<Vec<_>>(), vec![
        1233683848, 862516352, 1159784568, 1616057672, 412269392
    ]);
}
