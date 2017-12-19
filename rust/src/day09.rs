use common;
use std::str::Chars;
use std::iter::Peekable;

enum ParserState {
    NORMAL, GARBAGE
}

struct Parser<'a> {
    input: Peekable<Chars<'a>>,
    score: u32,
    garbage_count: u32,
    stack: Vec<u32>,
    state: ParserState,
}

impl<'a> Parser<'a> {
    fn score(input: &'a str) -> (u32, u32) {
        Self::new(input).get_score()
    }

    fn new(input: &'a str) -> Self {
        Parser {
            input: input.chars().peekable(),
            score: 0,
            garbage_count: 0,
            stack: vec![],
            state: ParserState::NORMAL,
        }
    }

    fn get_score(&mut self) -> (u32, u32) {
        while let Some(_) = self.input.peek() {
            match self.state {
                ParserState::NORMAL  => self.parse_normal(),
                ParserState::GARBAGE => self.parse_garbage()
            };
        }

        (self.score, self.garbage_count)
    }

    fn parse_normal(&mut self) {
        if self.consume('{') {
            let current_group_score = *self.stack.last().unwrap_or(&0);
            let new_group_score = current_group_score + 1;
            self.stack.push(new_group_score);
        } else if self.consume('}') {
            let current_group_score = self.stack.pop().expect("found a close group with no group");
            self.score += current_group_score;
        } else if self.consume('<') {
            self.state = ParserState::GARBAGE;
        } else if self.consume(',') {
            // nothing special to do for commas
        } else {
            panic!("found an unexpected character!")
        }
    }

    fn parse_garbage(&mut self) {
        if self.consume('!') {
            self.consume_any();
        } else if self.consume('>') {
            self.state = ParserState::NORMAL;
        } else {
            self.garbage_count += 1;
            self.consume_any();
        }
    }

    fn consume(&mut self, check: char) -> bool {
        match self.input.peek() {
            Some(&c) if c == check => {
                self.input.next();
                true
            },
            _ => false
        }
    }

    fn consume_any(&mut self) {
        self.input.next();
    }
}

pub fn run(_args: &[String]) {
    let input = common::get_input("./inputs/09.txt").expect("expected input 09.txt");
    let (score, garbage_count) = Parser::score(&input);
    println!("Part 1: Score is {}", score);
    println!("Part 2: Removed {} garbage chars", garbage_count);
}

#[test]
fn test_day9() {
    assert_eq!(Parser::score("{}"), (1, 0));
    assert_eq!(Parser::score("{{{}}}"), (6, 0));
    assert_eq!(Parser::score("{{},{}}"), (5, 0));
    assert_eq!(Parser::score("{{{},{},{{}}}}"), (16, 0));
    assert_eq!(Parser::score("{<a>,<a>,<a>,<a>}"), (1, 4));
    assert_eq!(Parser::score("{{<ab>},{<ab>},{<ab>},{<ab>}}"), (9, 8));
    assert_eq!(Parser::score("{{<!!>},{<!!>},{<!!>},{<!!>}}"), (9, 0));
    assert_eq!(Parser::score("{{<a!>},{<a!>},{<a!>},{<ab>}}"), (3, 17));
}
