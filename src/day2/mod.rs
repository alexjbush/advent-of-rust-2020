use lazy_static::lazy_static;
use regex::Regex;

use crate::Day;

pub struct Day2 {}

impl<'a> Day<'a> for Day2 {
    fn get_tasks(&self) -> Vec<(usize, &dyn Fn() -> String)> {
        vec![
            (1, &|| task(PolicyType::Part1)),
            (2, &|| task(PolicyType::Part2)),
        ]
    }

    fn get_day_number(&self) -> usize {
        2
    }
}

fn task(policy: PolicyType) -> String {
    get_input()
        .iter()
        .filter(|p| p.is_valid(&policy))
        .count()
        .to_string()
}

fn get_input() -> Vec<PasswordAndPolicy> {
    INPUT
        .lines()
        .map(|s| PasswordAndPolicy::from_string(&s.to_string()))
        .collect()
}

#[derive(Debug, PartialEq)]
struct PasswordAndPolicy {
    password: String,
    min: usize,
    max: usize,
    char: char,
}

enum PolicyType {
    Part1,
    Part2,
}

impl PasswordAndPolicy {
    fn is_valid(&self, policy: &PolicyType) -> bool {
        match policy {
            PolicyType::Part1 => {
                let count = self.password.chars().filter(|c| *c == self.char).count();
                count >= self.min && count <= self.max
            }
            PolicyType::Part2 => {
                (self.password.chars().nth(self.min - 1).unwrap() == self.char)
                    != (self.password.chars().nth(self.max - 1).unwrap() == self.char)
            }
        }
    }
    fn from_string(input: &String) -> PasswordAndPolicy {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)").unwrap();
        }
        let caps = REGEX.captures(input).unwrap();
        PasswordAndPolicy {
            min: caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            max: caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            char: caps.get(3).unwrap().as_str().chars().next().unwrap(),
            password: caps.get(4).unwrap().as_str().to_string(),
        }
    }
}

const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use crate::day2::{self, PasswordAndPolicy, PolicyType};

    #[test]
    fn test_is_valid_part1() {
        assert_eq!(
            day2::PasswordAndPolicy {
                password: "abcde".to_string(),
                min: 1,
                max: 3,
                char: 'a'
            }
            .is_valid(&PolicyType::Part1),
            true
        );
        assert_eq!(
            day2::PasswordAndPolicy {
                password: "cdefg".to_string(),
                min: 1,
                max: 3,
                char: 'b'
            }
            .is_valid(&PolicyType::Part1),
            false
        );
        assert_eq!(
            day2::PasswordAndPolicy {
                password: "ccccccccc".to_string(),
                min: 2,
                max: 9,
                char: 'c'
            }
            .is_valid(&PolicyType::Part1),
            true
        );
    }

    #[test]
    fn test_is_valid_part2() {
        assert_eq!(
            day2::PasswordAndPolicy {
                password: "abcde".to_string(),
                min: 1,
                max: 3,
                char: 'a'
            }
            .is_valid(&PolicyType::Part2),
            true
        );
        assert_eq!(
            day2::PasswordAndPolicy {
                password: "cdefg".to_string(),
                min: 1,
                max: 3,
                char: 'b'
            }
            .is_valid(&PolicyType::Part2),
            false
        );
        assert_eq!(
            day2::PasswordAndPolicy {
                password: "ccccccccc".to_string(),
                min: 2,
                max: 9,
                char: 'c'
            }
            .is_valid(&PolicyType::Part2),
            false
        );
    }

    #[test]
    fn test_from_string() {
        assert_eq!(
            PasswordAndPolicy::from_string(&"1-3 a: abcde".to_string()),
            day2::PasswordAndPolicy {
                password: "abcde".to_string(),
                min: 1,
                max: 3,
                char: 'a'
            }
        );
    }
}
