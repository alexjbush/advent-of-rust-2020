use std::str::Lines;

use crate::Day;

pub struct Day9 {}

impl<'a> Day<'a> for Day9 {
    fn get_tasks(&self) -> Vec<(usize, &dyn Fn() -> String)> {
        vec![(1, &task1), (2, &task2)]
    }

    fn get_day_number(&self) -> usize {
        9
    }
}

fn task1() -> String {
    let parsed = Stream::parse(INPUT.lines(), 25);
    parsed.find_invalid().unwrap().to_string()
}

fn task2() -> String {
    let parsed = Stream::parse(INPUT.lines(), 25);
    parsed.find_weakness().to_string()
}

struct Stream {
    preamble_length: usize,
    stream_content: Vec<u64>,
}

impl Stream {
    fn parse(lines: Lines, preamble_length: usize) -> Stream {
        let stream_content = lines
            .into_iter()
            .map(|l| l.parse::<u64>().unwrap())
            .collect();
        Stream {
            preamble_length,
            stream_content,
        }
    }

    fn find_invalid(&self) -> Option<&u64> {
        (self.preamble_length..self.stream_content.len())
            .find(|i| self.index_is_invalid(i))
            .map(|i| self.stream_content.get(i).unwrap())
    }

    fn index_is_invalid(&self, index: &usize) -> bool {
        let i_val = self.stream_content.get(*index).unwrap();
        for s in (index - self.preamble_length)..(index - 1) {
            let s_val = self.stream_content.get(s).unwrap();
            for e in (s + 1)..*index {
                let e_val = self.stream_content.get(e).unwrap();
                if s_val + e_val == *i_val {
                    return false;
                }
            }
        }
        return true;
    }

    fn find_matching_contiguous_numbers(&self, target: &u64) -> Vec<&u64> {
        let mut rolling_result: Vec<(u64, Vec<&u64>)> = vec![];
        for i in 0..self.stream_content.len() {
            let i_val = self.stream_content.get(i).unwrap();
            rolling_result.iter_mut().for_each(|e| {
                e.0 += i_val;
                e.1.push(i_val);
            });
            rolling_result.push((*i_val, vec![i_val]));
            match rolling_result.iter().find(|e| e.0 == *target) {
                Some(e) => return e.1.clone(),
                None => false,
            };
            rolling_result.retain(|e| e.0 < *target)
        }
        panic!("None found")
    }

    fn find_weakness(&self) -> u64 {
        let invalid_num = self.find_invalid().unwrap();
        let matching_contiguous = self.find_matching_contiguous_numbers(invalid_num);
        let min = matching_contiguous.iter().min().unwrap();
        let max = matching_contiguous.iter().max().unwrap();
        **min + **max
    }
}

const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1_INPUT: &str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    #[test]
    fn test_invalid() {
        let parsed = Stream::parse(TEST1_INPUT.lines(), 5);
        assert_eq!(*parsed.find_invalid().unwrap(), 127);
    }

    #[test]
    fn test_weakness() {
        let parsed = Stream::parse(TEST1_INPUT.lines(), 5);
        assert_eq!(parsed.find_weakness(), 62);
    }
}
