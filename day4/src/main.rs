use helpers::{print_day, print_solution, read_input};
use itertools::Itertools;

mod helpers;

pub fn part_one(input: &str) -> Option<usize> {
    let sum = input
        .split('\n')
        .filter(|line| {
            if line.is_empty() {
                return false;
            }

            let (elf1, elf2) = line.split_once(',').unwrap();
            let (elf1low, elf1high) = tuple_str_to_number(elf1.split_once('-').unwrap());
            let (elf2low, elf2high) = tuple_str_to_number(elf2.split_once('-').unwrap());

            if (elf1low <= elf2low && elf1high >= elf2high)
                || (elf2low <= elf1low && elf2high >= elf1high)
            {
                return true;
            }
            return false;
        })
        .collect_vec()
        .len();

    Some(sum)
}

fn tuple_str_to_number((str1, str2): (&str, &str)) -> (u8, u8) {
    return (
        str1.parse::<u8>().expect("{} is an invalid number"),
        str2.parse().expect("{} is an invalid number"),
    );
}

pub fn part_two(input: &str) -> Option<usize> {
    let sum = input
        .split('\n')
        .filter(|line| {
            if line.is_empty() {
                return false;
            }

            let (elf1, elf2) = line.split_once(',').unwrap();
            let (elf1low, elf1high) = tuple_str_to_number(elf1.split_once('-').unwrap());
            let (elf2low, elf2high) = tuple_str_to_number(elf2.split_once('-').unwrap());

            if (elf1high >= elf2low && elf1low <= elf2low)
                || (elf1low <= elf2high && elf1low >= elf2low)
            {
                return true;
            }
            return false;
        })
        .collect_vec()
        .len();

    Some(sum)
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
        assert_eq!(part_one(&input).unwrap(), 2);
    }
    #[test]
    fn test_part_two() {
        let input = read_example();
        assert_eq!(part_two(&input).unwrap(), 4);
    }
}
