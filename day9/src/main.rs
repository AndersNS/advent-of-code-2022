use std::{collections::HashSet, str::FromStr};

use helpers::{print_day, print_solution, read_input};
use itertools::Itertools;

mod helpers;

pub fn part_one(input: &str) -> Option<u32> {
    // (x,y)
    let mut head_current = (0, 0);
    let mut tail_current = (0, 0);
    let mut tail_visited: HashSet<(i32, i32)> = HashSet::new();
    tail_visited.insert(tail_current);
    for line in input.lines() {
        let (command, val) = line.split_once(" ").unwrap();
        let val = val.parse::<i32>().unwrap();

        for _ in 0..val {
            match command {
                "U" => head_current.1 -= 1,
                "D" => head_current.1 += 1,
                "L" => head_current.0 -= 1,
                "R" => head_current.0 += 1,
                _ => panic!("Unknown command"),
            }

            let diff_x = head_current.0 - tail_current.0;
            let diff_y = head_current.1 - tail_current.1;

            if diff_x.abs() > 1 || diff_y.abs() > 1 {
                tail_current.0 += diff_x.signum();
                tail_current.1 += diff_y.signum();
                tail_visited.insert(tail_current);
            }
        }
    }
    Some(tail_visited.len() as u32)
}
#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Coord {
    x: isize,
    y: isize,
}

pub fn part_two(input: &str) -> Option<u32> {
    // (x,y)
    let rope_start: (i32, i32) = (0, 0);
    let mut rope = vec![rope_start; 10];
    let mut tail_visited: HashSet<(i32, i32)> = HashSet::new();
    tail_visited.insert(rope_start);
    for line in input.lines() {
        let (command, val) = line.split_once(" ").unwrap();
        let val = val.parse::<i32>().unwrap();

        for _ in 0..val {
            match command {
                "U" => rope[0].1 -= 1,
                "D" => rope[0].1 += 1,
                "L" => rope[0].0 -= 1,
                "R" => rope[0].0 += 1,
                _ => panic!("Unknown command"),
            }

            for (head_rope_index, tail_rope_index) in (0..rope.len()).tuple_windows() {
                let diff_x = rope[head_rope_index].0 - rope[tail_rope_index].0;
                let diff_y = rope[head_rope_index].1 - rope[tail_rope_index].1;

                if diff_x.abs() > 1 || diff_y.abs() > 1 {
                    rope[tail_rope_index].0 += diff_x.signum();
                    rope[tail_rope_index].1 += diff_y.signum();
                }
            }
            tail_visited.insert(rope[rope.len() - 1]);
        }
    }

    Some(tail_visited.len() as u32)
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
        assert_eq!(part_one(&input).unwrap(), 13);
    }
    #[test]
    fn test_part_two() {
        let input = read_example();
        assert_eq!(part_two(&input).unwrap(), 36);
    }
}
