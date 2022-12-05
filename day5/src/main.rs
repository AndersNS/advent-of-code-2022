use std::{collections::HashMap, slice::SliceIndex, str::FromStr};

use helpers::{print_day, print_solution, read_input};
use itertools::Itertools;
use regex::Regex;

mod helpers;

struct Command {
    quant: usize,
    from: usize,
    to: usize,
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"move (\d*) from (\d*) to (\d*)").unwrap();
        let captures = re.captures(s).expect("{} Could not capture command");
        Ok(Self {
            quant: captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            from: captures.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            to: captures.get(3).unwrap().as_str().parse::<usize>().unwrap(),
        })
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let lines = input.lines().rev();
    let mut commands: Vec<Command> = vec![];
    let mut crates: Vec<Vec<&str>> = vec![];
    let mut crate_mode = false;
    for line in lines {
        if line.is_empty() {
            crate_mode = true;
            continue;
        }
        if crate_mode {
            if crates.len() == 0 {
                for _n in 0..(line.len() + 1) / 4 {
                    crates.push(vec![])
                }
            } else {
                for c in 0..(crates.len()) {
                    let start = 1 + (4 * c);
                    let end = start + 1;
                    let val = line.get(start..end).unwrap();
                    if !val.trim().is_empty() {
                        let aa = crates.get_mut(c).unwrap();
                        aa.push(val);
                    }
                }
            }
        } else {
            commands.push(line.parse::<Command>().unwrap());
        }
    }

    for command in commands.iter().rev() {
        let from = crates.get_mut(command.from - 1).unwrap();
        let mut items = from.drain(from.len() - command.quant..).collect_vec();
        items.reverse();
        let to = crates.get_mut(command.to - 1).unwrap();
        to.append(&mut items);
    }

    let mut streng = "".to_string();
    for mut cr in crates {
        streng = streng + cr.pop().unwrap();
    }

    Some(streng.to_string())
}

pub fn part_two(input: &str) -> Option<String> {
    let lines = input.lines().rev();
    let mut commands: Vec<Command> = vec![];
    let mut crates: Vec<Vec<&str>> = vec![];
    let mut crate_mode = false;
    for line in lines {
        if line.is_empty() {
            crate_mode = true;
            continue;
        }
        if crate_mode {
            if crates.len() == 0 {
                for _n in 0..(line.len() + 1) / 4 {
                    crates.push(vec![])
                }
            } else {
                for c in 0..(crates.len()) {
                    let start = 1 + (4 * c);
                    let end = start + 1;
                    let val = line.get(start..end).unwrap();
                    if !val.trim().is_empty() {
                        let aa = crates.get_mut(c).unwrap();
                        aa.push(val);
                    }
                }
            }
        } else {
            commands.push(line.parse::<Command>().unwrap());
        }
    }

    for command in commands.iter().rev() {
        let from = crates.get_mut(command.from - 1).unwrap();
        let mut items = from.drain(from.len() - command.quant..).collect_vec();
        // items.reverse(); // Part two
        let to = crates.get_mut(command.to - 1).unwrap();
        to.append(&mut items);
    }

    let mut streng = "".to_string();
    for cr in crates {
        streng = streng + cr[cr.len() - 1];
    }

    Some(streng.to_string())
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
        assert_eq!(part_one(&input).unwrap(), "CMZ".to_string());
    }
    #[test]
    fn test_part_two() {
        let input = read_example();
        assert_eq!(part_two(&input).unwrap(), "MCD".to_string());
    }
}
