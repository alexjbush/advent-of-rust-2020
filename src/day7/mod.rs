use crate::Day;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::str::Lines;

pub struct Day7 {}

impl<'a> Day<'a> for Day7 {
    fn get_tasks(&self) -> Vec<(usize, &dyn Fn() -> String)> {
        vec![(1, &task1), (2, &task2)]
    }

    fn get_day_number(&self) -> usize {
        7
    }
}

fn task1() -> String {
    let parsed = parse_input(INPUT.lines());
    let mut memo = HashMap::new();
    parsed
        .values()
        .filter(|b| b.can_contain("shiny gold", &parsed, &mut memo))
        .count()
        .to_string()
}

fn task2() -> String {
    let parsed = parse_input(INPUT.lines());
    let mut memo = HashMap::new();
    parsed
        .get("shiny gold")
        .unwrap()
        .contains(&parsed, &mut memo)
        .to_string()
}

struct Bag {
    color: String,
    contains: HashMap<String, u32>,
}

impl Bag {
    fn parse(input: &str) -> Bag {
        lazy_static! {
            static ref THIS_BAG_RE: Regex = Regex::new(r"^([a-z]+ [a-z]+)").unwrap();
            static ref CONTAINS_BAGS_RE: Regex = Regex::new(r"(\d+) ([a-z]+ [a-z]+)").unwrap();
        }
        let this_bag = THIS_BAG_RE
            .captures(input)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .to_string();
        let mut contains_bags: HashMap<String, u32> = HashMap::new();
        CONTAINS_BAGS_RE.captures_iter(input).for_each(|m| {
            contains_bags.insert(
                m.get(2).unwrap().as_str().to_string(),
                m.get(1).unwrap().as_str().parse().unwrap(),
            );
        });
        Bag {
            color: this_bag,
            contains: contains_bags,
        }
    }

    fn can_contain(
        &self,
        name: &str,
        rules: &HashMap<String, Bag>,
        memo: &mut HashMap<String, bool>,
    ) -> bool {
        self.contains.keys().any(|b| {
            name == b
                || match memo.get(b) {
                    Some(con) => con.to_owned(),
                    None => {
                        let con = rules.get(b).unwrap().can_contain(name, rules, memo);
                        memo.insert(b.to_string(), con);
                        con
                    }
                }
        })
    }

    fn contains(&self, rules: &HashMap<String, Bag>, memo: &mut HashMap<String, u32>) -> u32 {
        self.contains
            .iter()
            .map(|(b, c)| {
                c + c * match memo.get(b) {
                    Some(cnt) => *cnt,
                    None => {
                        let cnt = rules.get(b).unwrap().contains(rules, memo);
                        memo.insert(b.to_string(), cnt);
                        cnt
                    }
                }
            })
            .sum::<u32>()
    }
}

fn parse_input(input: Lines) -> HashMap<String, Bag> {
    input
        .map(|l| {
            let bag = Bag::parse(l);
            (bag.color.clone(), bag)
        })
        .collect()
}

const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::parse_input;

    const TEST1_INPUT: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    #[test]
    fn test_parse() {
        let parsed = parse_input(TEST1_INPUT.lines());
        assert_eq!(parsed.len(), 9);
    }

    #[test]
    fn test_can_contain() {
        let parsed = parse_input(TEST1_INPUT.lines());
        let mut memo = HashMap::new();
        let res = parsed
            .values()
            .filter(|b| b.can_contain("shiny gold", &parsed, &mut memo))
            .count();
        assert_eq!(res, 4);
    }

    #[test]
    fn test_contains() {
        let parsed = parse_input(TEST1_INPUT.lines());
        let mut memo = HashMap::new();
        let res = parsed
            .get("shiny gold")
            .unwrap()
            .contains(&parsed, &mut memo);
        assert_eq!(res, 32);
    }
}
