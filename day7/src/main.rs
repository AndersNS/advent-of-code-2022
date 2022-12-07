use helpers::{print_day, print_solution, read_input};
use std::collections::HashMap;

mod helpers;

#[derive(Debug)]
enum Command<'b> {
    LS,
    CD(&'b str),
}

impl Command<'_> {
    fn parse(line: &str) -> Option<Command> {
        let mut split = line.split(" ");
        if split.next().unwrap() != "$" {
            return None;
        }
        match split.next().unwrap() {
            "ls" => return Some(Command::LS),
            "cd" => return Some(Command::CD(split.next().unwrap())),
            _ => panic!("Unknown"),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sizes: HashMap<Vec<&str>, u32> = HashMap::new();
    let mut current: Vec<&str> = Vec::new();
    let mut current_size: u32 = 0;

    for line in input.lines() {
        match Command::parse(line) {
            Some(Command::CD(dir)) => {
                update_sizes(dir, &mut current, &mut sizes, &current_size);
                current_size = 0;
            }
            Some(Command::LS) => {}
            None => {
                let mut split = line.split(" ");
                let file_size = split.next().unwrap();
                if file_size != "dir" {
                    let file_size = file_size.parse::<u32>().unwrap();
                    current_size += file_size;
                }
            }
        }
    }

    update_sizes("", &mut current, &mut sizes, &current_size);

    let sum: u32 = sizes.values().filter(|d| **d < 100000).sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sizes: HashMap<Vec<&str>, u32> = HashMap::new();
    let mut current: Vec<&str> = Vec::new();
    let mut current_size: u32 = 0;

    for line in input.lines() {
        match Command::parse(line) {
            Some(Command::CD(dir)) => {
                update_sizes(dir, &mut current, &mut sizes, &current_size);
                current_size = 0;
            }
            Some(Command::LS) => {}
            None => {
                let mut words = line.split(" ");
                let maybe_file_size = words.next().unwrap();
                if maybe_file_size != "dir" {
                    let file_size = maybe_file_size
                        .parse::<u32>()
                        .expect("File size must be a number");
                    current_size += file_size;
                }
            }
        }
    }

    update_sizes("", &mut current, &mut sizes, &current_size);
    let free = 70_000_000 - sizes[&vec!["/"]];
    let to_clean = 30_000_000 - free;
    Some(*sizes.values().filter(|d| **d > to_clean).min().unwrap())
}

// Walks forward and updates the directory sizes for the parents as
// we go along
fn update_sizes<'b>(
    dir: &'b str,
    current: &mut Vec<&'b str>,
    sizes: &mut HashMap<Vec<&'b str>, u32>,
    current_size: &u32,
) {
    let size = current_size.clone();
    let mut partial_dir: Vec<&str> = Vec::new();
    for current_dir_part in current.iter() {
        partial_dir.push(current_dir_part);
        if !sizes.contains_key(&partial_dir) {
            sizes.insert(partial_dir.clone(), 0);
        }
        *sizes.get_mut(&partial_dir).unwrap() += size
    }
    if dir == ".." {
        current.pop();
    } else {
        current.push(dir);
    }
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
        assert_eq!(part_one(&input).unwrap(), 95437);
    }
    #[test]
    fn test_part_two() {
        let input = read_example();
        assert_eq!(part_two(&input), None);
    }
}
