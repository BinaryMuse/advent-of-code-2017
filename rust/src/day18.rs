use common;
use std::collections::HashMap;

pub fn run(_args: &[String]) {
    let input = common::get_input("./inputs/18.txt").expect("expected input 18.txt");
    let instructions: Vec<SoundcardInstruction> = input.trim().lines().map(|line| {
        SoundcardInstruction::parse(line).expect("expected to parse an instruction from a line")
    }).collect();
    let mut machine = Machine::new();
    machine.run(instructions);
    println!("Part 1: {:?}", machine.sound)
}

#[derive(Debug, PartialEq)]
enum RegisterValue {
    Register(String),
    Value(i64),
}

impl RegisterValue {
    fn parse(val: &str) -> Self {
        match val.parse::<i64>() {
            Ok(num) => RegisterValue::Value(num),
            _ => RegisterValue::Register(val.to_string()),
        }
    }
}

#[derive(Debug, PartialEq)]
enum SoundcardInstruction {
    Sound(RegisterValue),
    Set(String, RegisterValue),
    Add(String, RegisterValue),
    Mul(String, RegisterValue),
    Mod(String, RegisterValue),
    Recv(String),
    Jump(RegisterValue, RegisterValue),
}

impl SoundcardInstruction {
    fn parse(instr: &str) -> Option<Self> {
        let mut parts = instr.split(" ");

        match parts.next().unwrap() {
            "snd" => {
                let val = parts.next().unwrap();
                Some(SoundcardInstruction::Sound(RegisterValue::parse(val)))
            },
            "set" => {
                let reg = parts.next().unwrap();
                let val = parts.next().unwrap();
                Some(SoundcardInstruction::Set(reg.to_string(), RegisterValue::parse(val)))
            },
            "add" => {
                let reg = parts.next().unwrap();
                let val = parts.next().unwrap();
                Some(SoundcardInstruction::Add(reg.to_string(), RegisterValue::parse(val)))
            },
            "mul" => {
                let reg = parts.next().unwrap();
                let val = parts.next().unwrap();
                Some(SoundcardInstruction::Mul(reg.to_string(), RegisterValue::parse(val)))
            },
            "mod" => {
                let reg = parts.next().unwrap();
                let val = parts.next().unwrap();
                Some(SoundcardInstruction::Mod(reg.to_string(), RegisterValue::parse(val)))
            },
            "rcv" => {
                Some(SoundcardInstruction::Recv(parts.next().unwrap().to_string()))
            },
            "jgz" => {
                let val1 = parts.next().unwrap();
                let val2 = parts.next().unwrap();
                Some(SoundcardInstruction::Jump(
                    RegisterValue::parse(val1),
                    RegisterValue::parse(val2),
                ))
            }
            _ => None
        }
    }
}

#[test]
fn test_parse_soundcard_instruction() {
    use self::SoundcardInstruction::*;
    use self::RegisterValue::*;

    assert_eq!(SoundcardInstruction::parse("snd 14"), Some(Sound(Value(14))));
    assert_eq!(SoundcardInstruction::parse("snd x"), Some(Sound(Register("x".to_string()))));

    assert_eq!(SoundcardInstruction::parse("set a 13"),
               Some(Set("a".to_string(), Value(13))));
    assert_eq!(SoundcardInstruction::parse("set a x"),
               Some(Set("a".to_string(), Register("x".to_string()))));
}

struct Machine {
    registers: HashMap<String, i64>,
    sound: Option<i64>,
}

impl Machine {
    fn new() -> Self {
        Self { registers: HashMap::new(), sound: None }
    }

    fn run(&mut self, instructions: Vec<SoundcardInstruction>) {
        use self::SoundcardInstruction::*;
        let mut pointer: i64 = 0;

        while (pointer as usize) < instructions.len() {
            let instr = &instructions[pointer as usize];
            match instr {
                &Sound(ref val) => {
                    self.sound = Some(self.resolve_value(val));
                    pointer += 1;
                },
                &Set(ref reg, ref val) => {
                    let value = self.resolve_value(val);
                    self.set_register(reg, value);
                    pointer += 1;
                },
                &Add(ref reg, ref val) => {
                    let current = self.get_register(reg);
                    let new = current + self.resolve_value(val);
                    self.set_register(reg, new);
                    pointer += 1;
                },
                &Mul(ref reg, ref val) => {
                    let current = self.get_register(reg);
                    let new = current * self.resolve_value(val);
                    self.set_register(reg, new);
                    pointer += 1;
                },
                &Mod(ref reg, ref val) => {
                    let current = self.get_register(reg);
                    let new = current % self.resolve_value(val);
                    self.set_register(reg, new);
                    pointer += 1;
                },
                &Recv(ref reg) => {
                    let val = self.get_register(reg);
                    if val != 0 {
                        return;
                    }
                    pointer += 1;
                },
                &Jump(ref reg, ref val) => {
                    let value = self.resolve_value(reg);
                    if value > 0 {
                        pointer += self.resolve_value(val);
                    } else {
                        pointer += 1;
                    }
                },
            }
        }
    }

    fn resolve_value(&self, register_or_val: &RegisterValue) -> i64 {
        match register_or_val {
            &RegisterValue::Register(ref reg) => {
                let val = self.registers.get(reg).unwrap_or(&0);
                *val
            },
            &RegisterValue::Value(val) => val
        }
    }

    fn get_register(&self, register: &String) -> i64 {
        *self.registers.get(register).unwrap_or(&0)
    }

    fn set_register(&mut self, register: &String, value: i64) {
        self.registers.insert(register.clone(), value);
    }
}

#[test]
fn test_machine() {
    let input = "set a 1\nadd a 2\nmul a a\nmod a 5\nsnd a\nset a 0\nrcv a\njgz a -1\nset a 1\njgz a -2";
    let instructions: Vec<SoundcardInstruction> = input.trim().lines().map(|line| {
        SoundcardInstruction::parse(line).expect("expected to parse an instruction from a line")
    }).collect();
    let mut machine = Machine::new();
    machine.run(instructions);
    assert_eq!(machine.sound, Some(4));
}
