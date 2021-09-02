use std::iter;

use crate::Day;

pub struct Day1 {}

impl<'a> Day<'a> for Day1 {
    fn get_tasks(&self) -> Vec<(usize, &dyn Fn() -> String)> {
        vec![(1, &|| task(2)), (2, &|| task(3))]
    }

    fn get_day_number(&self) -> usize {
        1
    }
}

fn task(elems: usize) -> String {
    find_first_to_sum(&get_input(), 2020, elems)
        .unwrap()
        .to_string()
}

fn get_input() -> Vec<u32> {
    INPUT
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}

fn find_first_to_sum(input: &Vec<u32>, target: u32, num_elems: usize) -> Option<u64> {
    let super_input: Vec<&Vec<u32>> = iter::repeat(input).take(num_elems).collect();
    fn inner_loop(
        mut acc_input: Vec<&Vec<u32>>,
        sum_acc: u32,
        prod_acc: u64,
        target: u32,
    ) -> Option<u64> {
        match acc_input.pop() {
            None => {
                if sum_acc == target {
                    Some(prod_acc)
                } else {
                    None
                }
            }
            Some(i_input) => {
                for i in i_input {
                    if let Some(u) = (inner_loop)(
                        acc_input.clone(),
                        sum_acc + i,
                        prod_acc * u64::from(*i),
                        target,
                    ) {
                        return Some(u);
                    }
                }
                return None;
            }
        }
    }

    return inner_loop(super_input, 0, 1, target);
}

const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use crate::day1;

    #[test]
    fn test_find_first_to_sum() {
        assert_eq!(day1::find_first_to_sum(&vec![1, 2, 3], 0, 2), None);
        assert_eq!(
            day1::find_first_to_sum(&vec![1721, 979, 366, 299, 675, 1456], 2020, 2),
            Some(514579)
        );
    }
}
