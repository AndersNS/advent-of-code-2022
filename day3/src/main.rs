use helpers::{print_day, print_solution, read_input};
use itertools::{any, Itertools};

mod helpers;

pub fn part_one(input: &str) -> Option<u32> {
    let duplicates: u32 = input
        .split('\n')
        .map(|s| {
            let (split1, split2) = s.split_at(s.len() / 2);

            let mut s1_chars = split1.chars().collect_vec();
            s1_chars.dedup();
            let duplicates: Option<u32> = s1_chars
                .into_iter()
                .filter(|s1| return any(split2.chars(), |s2| s2 == *s1))
                .map(|c| {
                    return char_to_alpha_idx(c) as u32;
                })
                .max();

            if let Some(max) = duplicates {
                return max;
            } else {
                return 0;
            }
        })
        .sum();

    return Some(duplicates as u32);
}

fn char_to_alpha_idx(character: char) -> usize {
    let idx = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .find(character)
        .expect("Bad char for char_to_alpha_idx");
    return idx + 1;
}

pub fn part_two(input: &str) -> Option<u32> {
    let duplicates: u32 = input
        .split('\n')
        .filter(|x| !x.is_empty()) // why
        .chunks(3)
        .into_iter()
        .map(|s| {
            let v = s.collect_vec();
            let elf1 = v[0];
            let elf2 = v[1];
            let elf3 = v[2];

            let dupes = elf1
                .chars()
                .filter(|e1| {
                    return any(elf2.chars(), |e2| e2 == *e1) && any(elf3.chars(), |e3| e3 == *e1);
                })
                .dedup()
                .collect_vec();
            let f = dupes.first().expect("Should find one item");
            return char_to_alpha_idx(*f) as u32;
        })
        .sum();

    return Some(duplicates);
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
        assert_eq!(part_one(&input).unwrap(), 157);
    }
    #[test]
    fn test_part_two() {
        let input = read_example();
        assert_eq!(part_two(&input).unwrap(), 70);
    }
}
