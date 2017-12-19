extern crate num;

use common;
use std::str::FromStr;
use std::collections;
use std::collections::HashMap;
use self::num::Num;

#[derive(Debug, PartialEq)]
enum Operation {
    INC, DEC
}

#[derive(Debug, PartialEq)]
enum Comparator {
    EQ, NEQ, LT, LTE, GT, GTE
}

#[derive(Debug)]
struct Program<T: Num + Default + Copy + PartialOrd> {
    instructions: Vec<Instruction<T>>
}

impl<T: Num + Default + Copy + PartialOrd> Program<T> {
    fn parse(input: &str) -> Self {
        let instructions = input.lines().map(|line| Instruction::parse(line)).collect();
        Program::new(instructions)
    }

    fn new(instructions: Vec<Instruction<T>>) -> Self {
        Program { instructions }
    }
}

#[derive(Debug, PartialEq)]
struct Instruction<T: Num + Default + Copy + PartialOrd> {
    register: String,
    operation: Operation,
    amount: T,
    condition: Condition<T>
}

impl<T: Num + Default + Copy + PartialOrd> Instruction<T> {
    fn parse(text: &str) -> Self {
        let idx = text.find(" if ").unwrap();
        let (instr, cond) = (
            String::from_str(&text[..idx]).unwrap(),
            String::from_str(&text[idx+4..]).unwrap()
        );

        let condition: Condition<T> = Condition::parse(&cond);

        let mut parts = instr.split_whitespace();

        let register = parts.next().unwrap().to_string();
        let operation = match parts.next().unwrap() {
            "inc" => Operation::INC,
            "dec" => Operation::DEC,
            _ => panic!("Invalid operation")
        };
        // let amount = parts.next().unwrap().from_str_radix::<T>(10).ok().expect("couldn't parse amount");
        let amount = Num::from_str_radix(&parts.next().unwrap(), 10).ok().expect("couldn't parse amount");

        Instruction { register, operation, amount, condition }
    }
}

#[test]
fn test_instruction() {
    assert_eq!(
        Instruction::parse("b inc 5 if a > 1"),
        Instruction {
            register: "b".to_string(),
            operation: Operation::INC,
            amount: 5,
            condition: Condition {
                register: "a".to_string(),
                comparator: Comparator::GT,
                value: 1
            }
        }
    );
}

#[derive(Debug, PartialEq)]
struct Condition<T: Num + Default + Copy + PartialOrd> {
    register: String,
    comparator: Comparator,
    value: T
}

impl<T: Num + Default + Copy + PartialOrd> Condition<T> {
    fn parse(text: &str) -> Self {
        let mut parts = text.split_whitespace();

        let register = parts.next().unwrap().to_string();
        let comparator = match parts.next().unwrap() {
            "==" => Comparator::EQ,
            "!=" => Comparator::NEQ,
            "<"  => Comparator::LT,
            "<=" => Comparator::LTE,
            ">"  => Comparator::GT,
            ">=" => Comparator::GTE,
            _ => panic!("bad comparator")
        };
        let value = Num::from_str_radix(&parts.next().unwrap(), 10).ok().expect("couldn't parse compare amount");

        Condition { register, comparator, value }
    }
}

#[test]
fn test_condition() {
    assert_eq!(
        Condition::parse("a == -3"),
        Condition {
            register: "a".to_string(),
            comparator: Comparator::EQ,
            value: -3
        }
    );
    assert_eq!(
        Condition::parse("tux <= 42"),
        Condition {
            register: "tux".to_string(),
            comparator: Comparator::LTE,
            value: 42
        }
    );
}

#[derive(Debug)]
struct Machine<T: Num + Default + Copy + PartialOrd> {
    registers: HashMap<String, T>,
    highest_value: T
}

impl<T: Num + Default + Copy + PartialOrd> Machine<T> {
    fn new() -> Self {
        Machine { registers: HashMap::new(), highest_value: Default::default() }
    }

    fn run(&mut self, program: &Program<T>) {
        for instruction in program.instructions.iter() {
            self.run_instruction(instruction);
        }
    }

    fn run_instruction(&mut self, inst: &Instruction<T>) {
        let Instruction { ref register, ref operation, ref amount, ref condition } = *inst;
        let reg_str = register.to_string();

        // Touch the register now that we know about it
        self.registers.entry(reg_str.clone()).or_insert(Default::default());
        if self.check_condition(condition) {
            let default = Default::default();
            let current: T = *self.registers.get(&reg_str).unwrap_or(&default);
            let new_amount = match operation {
                &Operation::INC => current + *amount,
                &Operation::DEC => current - *amount
            };
            self.record_value(reg_str, new_amount);
        }
    }

    fn check_condition(&mut self, cond: &Condition<T>) -> bool {
        let Condition { ref register, ref comparator, ref value } = *cond;
        let reg_str = register.to_string();

        // Touch this register now that we know it exists.
        self.registers.entry(reg_str.clone()).or_insert(Default::default());
        let default = Default::default();
        let current: T = *self.registers.get(&reg_str).unwrap_or(&default);
        match comparator {
            &Comparator::EQ  => current == *value,
            &Comparator::NEQ => current != *value,
            &Comparator::LT  => current <  *value,
            &Comparator::LTE => current <= *value,
            &Comparator::GT  => current >  *value,
            &Comparator::GTE => current >= *value
        }
    }

    fn record_value(&mut self, register: String, value: T) {
        self.registers.insert(register, value);
        if value > self.highest_value {
            self.highest_value = value;
        }
    }

    #[cfg(test)]
    fn register_value(&self, register: &str) -> Option<&T> {
        self.registers.get(register)
    }

    fn registers(&self) -> collections::hash_map::Iter<String, T> {
        self.registers.iter()
    }

    fn highest_value(&self) -> T {
        self.highest_value
    }
}

pub fn run(_args: &[String]) {
    let input = common::get_input("./inputs/08.txt").expect("expected input 08.txt");
    let program: Program<i64> = Program::parse(&input);
    let mut machine = Machine::new();
    machine.run(&program);
    let (max_key, max_val) = machine.registers().max_by_key(|&(_key, val)| val).unwrap();
    println!("Part 1: {} in register {}", max_val, max_key);
    println!("Part 2: Highest ever value was {}", machine.highest_value());
}

#[test]
fn test_program() {
    let text = "b inc 5 if a > 1\n\
                a inc 1 if b < 5\n\
                c dec -10 if a >= 1\n\
                c inc -20 if c == 10";

    let program: Program<i64> = Program::parse(&text);
    let mut machine = Machine::new();
    machine.run(&program);

    assert_eq!(machine.register_value("a"), Some(&1));
    assert_eq!(machine.register_value("b"), Some(&0));
    assert_eq!(machine.register_value("c"), Some(&-10));
    assert_eq!(machine.highest_value(), 10);
}
