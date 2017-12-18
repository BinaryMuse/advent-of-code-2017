use common;

pub fn run(_args: &[String]) {
    let input: String = common::get_input("./inputs/01.txt").expect("expected input 01.txt");
    println!("Part 1: {}", get_sum(&input));
    println!("Part 2: {}", get_sum_matching_index(&input, |s, i| s.len() / 2 + i));
}

fn get_sum(input: &str) -> u32 {
    get_sum_matching_index(input, |_slice, idx| idx + 1)
}

fn get_sum_matching_index<F>(input: &str, mut get_next_index: F) -> u32
    where F: FnMut(&[u32], usize) -> usize
{
    let mut total = 0;
    let digits: Vec<u32> = input.chars().map(|d| d.to_digit(10).unwrap()).collect();

    for (i, digit) in digits.iter().enumerate() {
        let mut next_index: usize = get_next_index(&digits, i as usize);
        while next_index >= digits.len() {
            next_index -= digits.len();
        }
        let compare_to = digits[next_index];
        if *digit == compare_to {
            total += digit
        }
    }

    total
}

#[test]
fn test_day01_run() {
    assert_eq!(get_sum("1122"), 3);
    assert_eq!(get_sum("1111"), 4);
    assert_eq!(get_sum("1234"), 0);
    assert_eq!(get_sum("91212129"), 9);
    // Day 1 part 1
    assert_eq!(get_sum(&common::get_input("./inputs/01.txt").expect("expected input 01.txt")), 1047);

    let jump = |s: &[u32], i: usize| -> usize { s.len() / 2 + i };

    assert_eq!(get_sum_matching_index("1212", &jump), 6);
    assert_eq!(get_sum_matching_index("1221", &jump), 0);
    assert_eq!(get_sum_matching_index("123425", &jump), 4);
    assert_eq!(get_sum_matching_index("123123", &jump), 12);
    assert_eq!(get_sum_matching_index("12131415", &jump), 4);
    // Day 1 part 1
    assert_eq!(get_sum_matching_index(&common::get_input("./inputs/01.txt").expect("expected input 01.txt"), &jump), 982);
}
