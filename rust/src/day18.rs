use common;
use std::collections::HashMap;
use std::collections::VecDeque;

pub fn run(_args: &[String]) {
    let input = common::get_input("./inputs/18.txt").expect("expected input 18.txt");
    let instructions: Vec<Instruction> = input.trim().lines().map(|line| {
        Instruction::parse(line).expect("expected to parse an instruction from a line")
    }).collect();

    {
        let mut machine = Machine::new();
        machine.run(&instructions);
        println!("Part 1: {:?}", machine.sound);
    }

    {
        let mut prog0 = Program::new(0, &instructions);
        let mut prog1 = Program::new(1, &instructions);

        loop {
            let deadlock = prog0.is_waiting() && prog1.is_waiting();
            let terminated = prog0.is_terminated() && prog1.is_terminated();

            if deadlock || terminated {
                break
            }

            prog0.exec_next(&mut prog1);
            prog1.exec_next(&mut prog0);
        }

        println!("Part 2: {}", prog1.get_send_count());
    }
}

#[derive(Debug, PartialEq, Clone)]
enum RegisterValue {
    Register(String),
    Value(i64),
}

impl RegisterValue {
    fn parse(val: &str) -> Self {
        match val.parse::<i64>() {
            Ok(num) => RegisterValue::Value(num),
            _       => RegisterValue::Register(val.to_string()),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Instruction {
    Snd(RegisterValue),
    Set(String, RegisterValue),
    Add(String, RegisterValue),
    Mul(String, RegisterValue),
    Mod(String, RegisterValue),
    Rcv(String),
    Jmp(RegisterValue, RegisterValue),
}

impl Instruction {
    fn parse(instr: &str) -> Option<Self> {
        let mut parts = instr.split(" ");

        match parts.next().unwrap() {
            "snd" => {
                let val = parts.next().unwrap();
                Some(Instruction::Snd(RegisterValue::parse(val)))
            },
            "set" => {
                let reg = parts.next().unwrap();
                let val = parts.next().unwrap();
                Some(Instruction::Set(reg.to_string(), RegisterValue::parse(val)))
            },
            "add" => {
                let reg = parts.next().unwrap();
                let val = parts.next().unwrap();
                Some(Instruction::Add(reg.to_string(), RegisterValue::parse(val)))
            },
            "mul" => {
                let reg = parts.next().unwrap();
                let val = parts.next().unwrap();
                Some(Instruction::Mul(reg.to_string(), RegisterValue::parse(val)))
            },
            "mod" => {
                let reg = parts.next().unwrap();
                let val = parts.next().unwrap();
                Some(Instruction::Mod(reg.to_string(), RegisterValue::parse(val)))
            },
            "rcv" => {
                Some(Instruction::Rcv(parts.next().unwrap().to_string()))
            },
            "jgz" => {
                let val1 = parts.next().unwrap();
                let val2 = parts.next().unwrap();
                Some(Instruction::Jmp(
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
    use self::Instruction::*;
    use self::RegisterValue::*;

    assert_eq!(Instruction::parse("snd 14"), Some(Snd(Value(14))));
    assert_eq!(Instruction::parse("snd x"), Some(Snd(Register("x".to_string()))));

    assert_eq!(Instruction::parse("set a 13"),
               Some(Set("a".to_string(), Value(13))));
    assert_eq!(Instruction::parse("set a x"),
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

    fn run(&mut self, instructions: &Vec<Instruction>) {
        use self::Instruction::*;
        let mut pointer: i64 = 0;

        while (pointer as usize) < instructions.len() {
            let instr = &instructions[pointer as usize];
            match instr {
                &Snd(ref val) => {
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
                &Rcv(ref reg) => {
                    let val = self.get_register(reg);
                    if val != 0 {
                        return;
                    }
                    pointer += 1;
                },
                &Jmp(ref reg, ref val) => {
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
            &RegisterValue::Register(ref reg) => *self.registers.get(reg).unwrap_or(&0),
            &RegisterValue::Value(val)        => val
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
    let instructions: Vec<Instruction> = input.trim().lines().map(|line| {
        Instruction::parse(line).expect("expected to parse an instruction from a line")
    }).collect();
    let mut machine = Machine::new();
    machine.run(&instructions);
    assert_eq!(machine.sound, Some(4));
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum ProgramState {
    Running,
    Waiting,
    Terminated,
}

struct Program<'a> {
    _number: i64,
    machine: Machine,
    mailbox: VecDeque<i64>,
    instructions: &'a [Instruction],
    state: ProgramState,
    pointer: i64,
    send_count: usize,
}

impl<'a> Program<'a> {
    fn new(number: i64, instructions: &'a Vec<Instruction>) -> Self {
        let mut prog = Program {
            _number: number,
            machine: Machine::new(),
            mailbox: VecDeque::new(),
            instructions,
            state: ProgramState::Running,
            pointer: 0,
            send_count: 0,
        };

        prog.machine.set_register(&"p".to_string(), number);

        prog
    }

    pub fn exec_next(&mut self, other_prog: &mut Program) {
        if self.is_terminated() || self.is_waiting() {
            return
        }

        let instr = self.instructions.get(self.pointer as usize);

        use self::Instruction::*;
        match instr.clone() {
            Some(&Snd(ref val)) => {
                let value = self.machine.resolve_value(val);
                self.send_value(other_prog, value);
                self.pointer += 1;
            },
            Some(&Set(ref reg, ref val)) => {
                let value = self.machine.resolve_value(val);
                self.machine.set_register(reg, value);
                self.pointer += 1;
            },
            Some(&Add(ref reg, ref val)) => {
                let current = self.machine.get_register(reg);
                let new = current + self.machine.resolve_value(val);
                self.machine.set_register(reg, new);
                self.pointer += 1;
            },
            Some(&Mul(ref reg, ref val)) => {
                let current = self.machine.get_register(reg);
                let new = current * self.machine.resolve_value(val);
                self.machine.set_register(reg, new);
                self.pointer += 1;
            },
            Some(&Mod(ref reg, ref val)) => {
                let current = self.machine.get_register(reg);
                let new = current % self.machine.resolve_value(val);
                self.machine.set_register(reg, new);
                self.pointer += 1;
            },
            Some(&Rcv(ref reg)) => {
                if let Some(value) = self.receive_value() {
                    self.machine.set_register(reg, value);
                    self.pointer += 1;
                }
            },
            Some(&Jmp(ref reg, ref val)) => {
                let value = self.machine.resolve_value(reg);
                if value > 0 {
                    self.pointer += self.machine.resolve_value(val);
                } else {
                    self.pointer += 1;
                }
            },
            None => {
                self.state = ProgramState::Terminated;
                return
            }
        }
    }

    fn push_mailbox(&mut self, value: i64) {
        self.mailbox.push_back(value);
    }

    fn send_value(&mut self, other_program: &mut Program, value: i64) {
        self.send_count += 1;
        other_program.push_mailbox(value);
    }

    fn receive_value(&mut self) -> Option<i64> {
        self.state = ProgramState::Waiting;
        if self.mailbox.len() > 0 {
            let val = self.mailbox.pop_front().unwrap();
            self.state = ProgramState::Running;
            return Some(val);
        }

        None
    }

    fn is_waiting(&self) -> bool {
        self.state == ProgramState::Waiting && self.mailbox.len() == 0
    }

    fn is_terminated(&self) -> bool {
        self.state == ProgramState::Terminated
    }

    fn get_send_count(&self) -> usize {
        self.send_count
    }
}
