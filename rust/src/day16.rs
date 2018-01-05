use common;
use self::DanceMove::*;

pub fn run(_args: &[String]) {
    let input = common::get_input("./inputs/16.txt").expect("expected input 16.txt");
    let mut programs: Vec<char> = "abcdefghijklmnop".chars().collect();
    let moves: Vec<&str> = input.trim().split(",").collect();
    dance(&mut programs, &moves);
    println!("Part 1: order is {}", programs.into_iter().collect::<String>());
}

#[derive(Debug)]
enum DanceMove {
    Spin(u32),
    Exchange(usize, usize),
    Partner(char, char),
}

fn parse_move(dance_move: &&str) -> Option<DanceMove> {
    let first = dance_move.chars().next();
    let rest: String = dance_move.chars().skip(1).collect();

    match first {
        Some('s') => {
            let num: u32 = rest.parse().unwrap();
            Some(Spin(num))
        },
        Some('x') => {
            let parts: Vec<usize> = rest.split("/").map(|s| s.parse().unwrap()).collect();
            Some(Exchange(parts[0], parts[1]))
        },
        Some('p') => {
            let mut programs = rest.split("/");
            let first: char = programs.next().unwrap().chars().next().unwrap();
            let second: char = programs.next().unwrap().chars().next().unwrap();
            Some(Partner(first, second))
        }
        _ => None
    }
}

fn dance(programs: &mut Vec<char>, moves: &[&str]) {
    let steps = moves.iter().map(parse_move);

    for step in steps {
        match step {
            Some(Spin(amount)) => {
                for _ in 0..amount {
                    let program = programs.pop().unwrap();
                    programs.insert(0, program);
                }
            },
            Some(Exchange(pos1, pos2)) => {
                programs.swap(pos1, pos2);
            },
            Some(Partner(prog1, prog2)) => {
                let pos1 = programs.iter().position(|&p| p == prog1).expect("couldn't find program");
                let pos2 = programs.iter().position(|&p| p == prog2).expect("couldn't find program");
                programs.swap(pos1, pos2);
            },
            None => {
                panic!("No dance move found")
            }
        }
    }
}

#[test]
fn test_dance() {
    let mut programs = vec!['a', 'b', 'c', 'd', 'e'];
    let moves = vec!["s1", "x3/4", "pe/b"];
    dance(&mut programs, &moves);
    assert_eq!(programs, vec!['b', 'a', 'e', 'd', 'c']);
}
