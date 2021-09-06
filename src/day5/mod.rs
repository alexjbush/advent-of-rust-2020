use crate::Day;
pub struct Day5 {}

impl<'a> Day<'a> for Day5 {
    fn get_tasks(&self) -> Vec<(usize, &dyn Fn() -> String)> {
        vec![(1, &task1), (2, &task2)]
    }

    fn get_day_number(&self) -> usize {
        5
    }
}

fn task1() -> String {
    get_ticket_ids().max().unwrap().to_string()
}

fn task2() -> String {
    let mut ids = get_ticket_ids().collect::<Vec<u32>>();
    ids.sort();
    let mut prev: Option<u32> = None;
    for id in ids {
        if id > 1 && prev == Some(id - 2) {
            return (id - 1).to_string();
        } else {
            prev = Some(id)
        }
    }
    "ERROR".to_string()
}

fn get_ticket_ids() -> Box<dyn Iterator<Item = u32>> {
    Box::new(INPUT.lines().map(|p| Pass::decode_pass(p).get_id()))
}

const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct Pass {
    row: u32,
    column: u32,
}

impl PartialEq for Pass {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.column == other.column
    }
}

impl Pass {
    fn decode_pass(code: &str) -> Pass {
        Pass {
            row: Pass::sub_pass(&code[0..7], 'B'),
            column: Pass::sub_pass(&code[7..10], 'R'),
        }
    }

    fn sub_pass(code: &str, upper_char: char) -> u32 {
        u32::from_str_radix(
            code.chars()
                .map(|b| if b == upper_char { '1' } else { '0' })
                .collect::<String>()
                .as_str(),
            2,
        )
        .unwrap()
    }

    fn get_id(&self) -> u32 {
        (self.row * 8) + self.column
    }
}

#[cfg(test)]
mod tests {
    use crate::day5::{self, Pass};

    #[test]
    fn test_sub_pass() {
        assert_eq!(day5::Pass::sub_pass("RLR", 'R'), 5);
        assert_eq!(day5::Pass::sub_pass("FBFBBFF", 'B'), 44);
    }

    #[test]
    fn test_decode_pass() {
        assert_eq!(
            day5::Pass::decode_pass("FBFBBFFRLR"),
            Pass { row: 44, column: 5 }
        );
    }
}
