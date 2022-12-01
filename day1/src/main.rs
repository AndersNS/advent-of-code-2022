use std::collections::BinaryHeap;

use helpers::{print_day, print_solution, read_input};

mod helpers;

pub fn part_one(input: &str) -> Option<u32> {
    let res = input
        .split('\n')
        .fold((0 as u32, 0 as u32), |(max, acc), x| {
            if x.is_empty() {
                if acc > max {
                    return (acc, 0 as u32);
                }
                return (max, 0);
            }
            return (max, acc + x.parse::<u32>().unwrap());
        });

    Some(res.0)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut res = input
        .split('\n')
        .fold((BinaryHeap::<u32>::new(), 0 as u32), |(mut btree, y), x| {
            if x.is_empty() {
                btree.push(y);
                return (btree, 0);
            }

            return (btree, y + x.parse::<u32>().unwrap());
        })
        .0;

    Some(res.pop()? + res.pop()? + res.pop()?)
}

fn main() {
    let input = read_input();
    print_day();
    print_solution(1, part_one, &input);
    print_solution(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::read_example;

    #[test]
    fn test_part_one() {
        let input = read_example();
        assert_eq!(part_one(&input).unwrap(), 24000);
    }
    #[test]
    fn test_part_two() {
        let input = read_example();
        assert_eq!(part_two(&input).unwrap(), 45000);
    }
}
