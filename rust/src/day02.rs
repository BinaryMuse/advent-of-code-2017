use common;
use std::str::FromStr;

type Spreadsheet<T> = Vec<Vec<T>>;

pub fn run(_args: &[String]) {
    let input = common::get_input("./inputs/02.txt").expect("expected input 02.txt");
    let sheet = text_to_ss(&input);
    println!("Part 1: {}", checksum(&sheet, row_checksum_hi_lo));
    println!("Part 2: {}", checksum(&sheet, row_checksum_div));
}

fn text_to_ss(input: &str) -> Spreadsheet<i32> {
    input.lines().map(|row| {
        row.split(char::is_whitespace).map(|c| i32::from_str(&c).unwrap()).collect()
    }).collect()
}

#[test]
fn test_text_to_ss() {
    assert_eq!(
        text_to_ss("1 2 3\n4 5 6"),
        vec![vec![1, 2, 3], vec![4, 5, 6]]
    );
}

fn checksum<F>(sheet: &Spreadsheet<i32>, mut row_checksum: F) -> i32
    where F: FnMut(&[i32]) -> i32
{
    sheet.iter().fold(0, |acc, row| {
        acc + row_checksum(row)
    })
}

#[test]
fn test_checksum() {
    let sheet1 = text_to_ss("5 1 9 5\n7 5 3\n2 4 6 8");
    assert_eq!(checksum(&sheet1, row_checksum_hi_lo), 18);

    let sheet2 = text_to_ss("5 9 2 8\n9 4 7 3\n3 8 6 5");
    assert_eq!(checksum(&sheet2, row_checksum_div), 9);
}

fn row_checksum_hi_lo(row: &[i32]) -> i32 {
    let mut clone = row.to_vec();
    clone.sort();
    clone.first().and_then(|smallest| {
        clone.last().and_then(|largest| {
            Some(largest - smallest)
        })
    }).unwrap_or(0)
}

fn row_checksum_div(row: &[i32]) -> i32 {
    let pair = find_div(row).expect("fatal error: could not find evently divisible numbers");
    pair.0 / pair.1
}

fn find_div(v: &[i32]) -> Option<(i32, i32)> {
    for (i, val1) in v.iter().enumerate() {
        for val2 in &v[i+1..] {
            if val1 % val2 == 0 {
                return Some((*val1, *val2));
            } else if val2 % val1 == 0 {
                return Some((*val2, *val1));
            }
        }
    }

    None
}

#[test]
fn test_find_div() {
    assert_eq!(find_div(&vec![3, 2, 7, 9]), Some((9, 3)));
    assert_eq!(find_div(&vec![9, 2, 7, 3]), Some((9, 3)));
    assert_eq!(find_div(&vec![3, 7, 13]), None);
}

#[test]
fn test_day02_run() {
    let sheet = text_to_ss(&common::get_input("./inputs/02.txt").expect("expected input 02.txt"));
    // Part 1
    assert_eq!(checksum(&sheet, row_checksum_hi_lo), 44216);
    // Part 2
    assert_eq!(checksum(&sheet, row_checksum_div), 320);
}
