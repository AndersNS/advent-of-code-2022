use helpers::{print_day, print_solution, read_input};
use std::collections::HashSet;

mod helpers;

pub fn part_one(input: &str) -> Option<usize> {
    let mut pos: usize = 0;
    let mut map: HashSet<char>;

    for i in 0..input.len() - 4 {
        map = (i..i + 4).map(|s| input.chars().nth(s).unwrap()).collect();

        if map.len() == 4 {
            pos = i + 4;
            break;
        }

        map.clear();
    }

    return Some(pos);
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut pos: usize = 0;
    let mut map: HashSet<char>;

    for i in 0..input.len() - 14 {
        map = (i..i + 14).map(|s| input.chars().nth(s).unwrap()).collect();

        if map.len() == 14 {
            pos = i + 14;
            break;
        }

        map.clear();
    }

    return Some(pos);
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
    fn test_part_one_one() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(part_one(&input).unwrap(), 5);
    }

    #[test]
    fn test_part_one_two() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(part_one(&input).unwrap(), 6);
    }

    #[test]
    fn test_part_one_three() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(part_one(&input).unwrap(), 10);
    }

    #[test]
    fn test_part_one_four() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(part_one(&input).unwrap(), 11);
    }

    #[test]
    fn test_part_two_one() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(part_two(&input).unwrap(), 19);
    }
}
