use crate::Day;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::str::Split;

pub struct Day6 {}

impl<'a> Day<'a> for Day6 {
    fn get_tasks(&self) -> Vec<(usize, &dyn Fn() -> String)> {
        vec![(1, &task1), (2, &task2)]
    }

    fn get_day_number(&self) -> usize {
        6
    }
}

fn task1() -> String {
    get_input()
        .map(|f| {
            let mut qs = f.replace("\n", "").chars().collect::<Vec<char>>();
            qs.sort();
            qs.dedup();
            qs.len()
        })
        .sum::<usize>()
        .to_string()
}

fn task2() -> String {
    get_input()
        .map(|f| {
            f.lines()
                .map(|x| HashSet::from_iter(x.chars().into_iter()))
                .reduce(|a: HashSet<char>, b| {
                    let r: HashSet<&char> = HashSet::from_iter(a.intersection(&b));
                    HashSet::from_iter(r.iter().map(|x| *x.to_owned()))
                })
                .unwrap()
                .len()
        })
        .sum::<usize>()
        .to_string()
}

fn get_input<'a>() -> Split<'a, &'a str> {
    INPUT.split("\n\n")
}

const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {

    #[test]
    fn test_1() {
        assert_eq!(5, 5);
    }
}
