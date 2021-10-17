use std::str::Lines;

use crate::Day;

pub struct Day8 {}

impl<'a> Day<'a> for Day8 {
    fn get_tasks(&self) -> Vec<(usize, &dyn Fn() -> String)> {
        vec![(1, &task1), (2, &task2)]
    }

    fn get_day_number(&self) -> usize {
        8
    }
}

fn task1() -> String {
    let mut parsed = Instructions::parse(INPUT.lines());
    parsed.iterate_until_terminated();
    parsed.acc.to_string()
}

fn task2() -> String {
    let mut parsed = Instructions::parse(INPUT.lines());
    parsed.iterate_until_terminated_with_corruption();
    parsed.acc.to_string()
}

const INPUT: &str = include_str!("input.txt");

enum Instruction {
    NoOp(i32),
    Acc(i32),
    Jump(i32),
}
use crate::day8::Instruction::*;
impl Instruction {
    fn parse(inst: &str) -> Instruction {
        let op = &inst[0..3];
        let sign = &inst[4..5];
        let num = inst[5..]
            .parse::<i32>()
            .map(|v| if sign == "+" { v } else { v * -1 })
            .unwrap();
        match op {
            "nop" => NoOp(num),
            "acc" => Acc(num),
            "jmp" => Jump(num),
            _ => panic!(""),
        }
    }
}

struct Instructions {
    acc: i32,
    line: usize,
    instructions: Vec<(Instruction, bool)>,
}

impl Instructions {
    fn parse(input: Lines) -> Instructions {
        Instructions {
            acc: 0,
            line: 0,
            instructions: input.map(|l| (Instruction::parse(l), false)).collect(),
        }
    }

    fn get_state(&self) -> NextState {
        if self.line >= self.instructions.len() {
            NextState::Terminated
        } else if self.instructions.get(self.line).unwrap().1 {
            NextState::Loop
        } else {
            NextState::Valid
        }
    }

    fn next(&mut self) -> () {
        let this = self.instructions.get_mut(self.line).unwrap();
        this.1 = true;
        match this.0 {
            NoOp(_) => self.line += 1,
            Acc(num) => {
                self.acc += num;
                self.line += 1;
            }
            Jump(num) => {
                let next = (self.line as i32) + num;
                self.line = next as usize;
            }
        }
    }

    fn iterate_until_terminated(&mut self) -> bool {
        loop {
            match self.get_state() {
                NextState::Loop => return false,
                NextState::Terminated => return true,
                NextState::Valid => self.next(),
            }
        }
    }

    fn negate_instruction_at_line(&mut self, line: usize) -> bool {
        let this = self.instructions.get_mut(line).unwrap();
        match this.0 {
            NoOp(num) => {
                this.0 = Jump(num);
                return true;
            }
            Acc(_) => {
                return false;
            }
            Jump(num) => {
                this.0 = NoOp(num);
                return true;
            }
        }
    }

    fn reset_state(&mut self) -> () {
        self.instructions.iter_mut().for_each(|e| e.1 = false);
        self.acc = 0;
        self.line = 0;
    }

    fn iterate_until_terminated_with_corruption(&mut self) -> () {
        let mut mutate_line: usize = 0;
        let mut clean_run: bool = false;
        loop {
            let changed = self.negate_instruction_at_line(mutate_line);
            if !changed && !clean_run {
                clean_run = true;
            } else if !changed {
                mutate_line += 1;
                continue;
            }
            let res = self.iterate_until_terminated();
            self.negate_instruction_at_line(mutate_line);
            mutate_line += 1;
            if res {
                break;
            }
            self.reset_state();
        }
    }
}

pub enum NextState {
    Valid,
    Loop,
    Terminated,
}

#[cfg(test)]
mod tests {
    use crate::day8::Instructions;

    const TEST1_INPUT: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    const TEST2_INPUT: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
nop -4
acc +6";

    #[test]
    fn test_parse() {
        let parsed = Instructions::parse(TEST1_INPUT.lines());
        assert_eq!(parsed.instructions.len(), 9);
    }

    #[test]
    fn test_task_1() {
        let mut parsed = Instructions::parse(TEST1_INPUT.lines());
        parsed.iterate_until_terminated();
        assert_eq!(parsed.acc, 5);
    }

    #[test]
    fn test_task_2_loop() {
        let mut parsed = Instructions::parse(TEST1_INPUT.lines());
        let res = parsed.iterate_until_terminated();
        assert_eq!(parsed.acc, 5);
        assert!(!res)
    }

    #[test]
    fn test_task_2_terminate() {
        let mut parsed = Instructions::parse(TEST2_INPUT.lines());
        let res = parsed.iterate_until_terminated();
        assert_eq!(parsed.acc, 8);
        assert!(res)
    }
}
