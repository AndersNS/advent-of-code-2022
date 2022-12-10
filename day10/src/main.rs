use std::{collections::HashMap, iter};

use helpers::{print_day, print_solution, read_input};
use itertools::Itertools;

mod helpers;

pub fn part_one(input: &str) -> Option<i32> {
    let mut x = 1i32;
    let mut cycle = 0i32;
    let cycles: Vec<i32> = vec![20, 60, 100, 140, 180, 220];
    let mut signal_strengths: HashMap<i32, i32> = HashMap::new();
    for line in input.lines() {
        let mut split = line.split(" ");
        let command = split.next().unwrap();
        if command == "noop" {
            cycle += 1;
            signal_strengths.insert(cycle, x * cycle);
        }
        if command == "addx" {
            cycle += 1;
            signal_strengths.insert(cycle, x * cycle);
            cycle += 1;
            signal_strengths.insert(cycle, x * cycle);
            x += split.next().unwrap().parse::<i32>().unwrap();
        }
    }

    Some(
        cycles
            .iter()
            .map(|cycle| signal_strengths.get(&cycle).unwrap().clone())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<String> {
    let mut x = 1i32;
    let mut cycle = 0i32;
    let mut signal_strengths: HashMap<i32, i32> = HashMap::new();
    for line in input.lines() {
        let mut split = line.split(" ");
        let command = split.next().unwrap();
        if command == "noop" {
            cycle += 1;
            signal_strengths.insert(cycle, x);
        }
        if command == "addx" {
            cycle += 1;
            signal_strengths.insert(cycle, x);
            cycle += 1;
            signal_strengths.insert(cycle, x);
            x += split.next().unwrap().parse::<i32>().unwrap();
        }
    }
    let mut crt: [[char; 40]; 6] = [[' '; 40]; 6];

    for cycle in signal_strengths {
        let x = cycle.1;
        let row = cycle.0 - 1 % 40;
        let range = (x - 1)..=(x + 1);
        if range.contains(&((row) % 40)) {
            crt[((row) / 40) as usize][((row) % 40) as usize] = 'x';
        }
    }
    Some(crt.map(|row| row.iter().collect::<String>()).join("\n"))
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
        assert_eq!(part_one(&input).unwrap(), 13140);
    }

    #[test]
    fn test_part_two() {
        let input = read_example();
        assert_eq!(part_two(&input), None);
    }
}
