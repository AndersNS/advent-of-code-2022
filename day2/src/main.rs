use std::str::FromStr;

use helpers::{print_day, print_solution, read_input};

mod helpers;

#[derive(Clone, Copy)]
enum RpsEnum {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

impl FromStr for RpsEnum {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(RpsEnum::Rock),
            "B" | "Y" => Ok(RpsEnum::Paper),
            "C" | "Z" => Ok(RpsEnum::Scissors),
            _ => Err(format!("'{}' is not a valid value", s)),
        }
    }
}

struct StrategyLine {
    theirs: RpsEnum,
    response: RpsEnum,
}

impl StrategyLine {
    fn new(theirs: RpsEnum, response: RpsEnum) -> Self {
        Self { theirs, response }
    }

    fn points_one(self: &Self) -> i8 {
        let theirs = self.theirs as i8;
        let response = self.response as i8;

        let diff = (theirs - 1) - (response - 1);

        return match diff {
            0 => 3 + response + 1,
            -1 | 2 => 6 + response + 1,
            _ => 0 + response + 1,
        };
    }

    fn points_two(self: &Self) -> i8 {
        let theirs = self.theirs as i8;
        let response = self.response as i8;

        return match response {
            0 => ((theirs + 2) % 3) + 1,
            1 => theirs + 3 + 1,
            _ => ((theirs + 1) % 3) + 6 + 1,
        };
    }
}

impl FromStr for StrategyLine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res: Vec<RpsEnum> = s
            .split(' ')
            .map(|s| s.parse::<RpsEnum>().unwrap())
            .collect::<Vec<RpsEnum>>();

        return Ok(StrategyLine::new(res[0], res[1]));
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = input
        .split('\n')
        .filter(|x| !x.is_empty()) // why
        .map(|x| x.parse::<StrategyLine>().unwrap())
        .fold(0, |acc, x| {
            return acc + x.points_one() as u32;
        });

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = input
        .split('\n')
        .filter(|x| !x.is_empty()) // why
        .map(|x| x.parse::<StrategyLine>().unwrap())
        .fold(0, |acc, x| {
            return acc + x.points_two() as u32;
        });

    Some(result)
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
        assert_eq!(part_one(&input).unwrap(), 15);
    }
    #[test]
    fn test_part_two() {
        let input = read_example();
        assert_eq!(part_two(&input).unwrap(), 12);
    }
}
