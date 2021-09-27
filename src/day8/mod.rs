use std::str::Lines;

use crate::Day;

pub struct Day8 {}

impl<'a> Day<'a> for Day8 {
    fn get_tasks(&self) -> Vec<(usize, &dyn Fn() -> String)> {
        vec![(1, &task1)]
    }

    fn get_day_number(&self) -> usize {
        8
    }
}

fn task1() -> String {
    let mut parsed = Instructions::parse(INPUT.lines());
    parsed.iterate_until_loop();
    parsed.acc.to_string()
}

const INPUT: &str = include_str!("input.txt");

enum Instruction {
    NoOp,
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
            "nop" => NoOp,
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

    fn next_is_valid(&self) -> bool {
        !self.instructions.get(self.line).unwrap().1
    }

    fn next(&mut self) -> () {
        let this = self.instructions.get_mut(self.line).unwrap();
        this.1 = true;
        match this.0 {
            NoOp => self.line += 1,
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

    fn iterate_until_loop(&mut self) -> () {
        loop {
            if self.next_is_valid() {
                self.next();
            } else {
                break;
            }
        }
    }
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

    #[test]
    fn test_parse() {
        let parsed = Instructions::parse(TEST1_INPUT.lines());
        assert_eq!(parsed.instructions.len(), 9);
    }

    #[test]
    fn test_task_1() {
        let mut parsed = Instructions::parse(TEST1_INPUT.lines());
        parsed.iterate_until_loop();
        assert_eq!(parsed.acc, 5);
    }
}
