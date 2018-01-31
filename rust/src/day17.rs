pub fn run(_args: &[String]) {
    // Hard-coded input
    let mut v = Vec::with_capacity(50_000_001);
    v.push(0);
    spinlock(&mut v, 349, 1, 2017);
    let idx = v.iter().position(|&n| n == 2017).expect("couldn't find position of 2017");
    let n = v[idx+1];
    println!("Part 1: The number after 2017 is {}", n);

    let m = spinlock_for_index(1, 349, 1, 50_000_000);
    println!("Part 1: The number after 0 is {:?}", m);
}

fn spinlock(v: &mut Vec<u64>, steps: usize, start: u64, end: u64) {
    let mut pos = 0;

    for i in start..end+1 {
        let forward = steps % v.len();
        let mut idx = pos + forward;
        while idx >= v.len() {
            idx -= v.len()
        }
        v.insert(idx + 1, i as u64);
        pos = idx + 1;
    }
}

fn spinlock_for_index(index: usize, steps: usize, start: u64, end: u64) -> Option<u64> {
    let mut pos = 0;
    let mut answer: Option<u64> = None;

    for i in start..end+1 {
        let list_size = i as usize;
        let mut idx = pos + steps;
        while idx >= list_size {
            idx -= list_size;
        }

        if idx + 1 == index {
            answer = Some(i);
        }

        pos = idx + 1;
    }

    answer
}

#[test]
fn test_spinlock() {
    let mut spin = vec![0];
    spinlock(&mut spin, 3, 1, 2017);
    let idx = spin.iter().position(|&n| n == 2017).expect("couldn't find position of 2017");
    let n = spin[idx+1];
    assert_eq!(n, 638);
}
