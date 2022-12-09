use helpers::{print_day, print_solution, read_input};
use itertools::Itertools;

mod helpers;

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<u32>> = vec![];

    let lines = input.lines().collect_vec();

    for y in 0..lines.len() {
        let chars = lines[y].chars().collect_vec();
        let mut xaxis: Vec<u32> = vec![];
        for x in 0..chars.len() {
            let num = chars[x]
                .to_string()
                .parse::<u32>()
                .expect("Must be a valid u8");
            xaxis.push(num)
        }
        grid.push(xaxis);
    }

    let mut visible_count = 0;
    let x_max = grid[0].len();
    for (y, row) in grid.iter().enumerate() {
        if y == 0 || y == grid.len() - 1 {
            visible_count += x_max;
        } else {
            for (x, _) in row.iter().enumerate() {
                if visible(&grid, x_max, x, y) {
                    visible_count += 1
                }
            }
        }
    }
    Some(visible_count as u32)
}

fn visible(grid: &Vec<Vec<u32>>, x_max: usize, x: usize, y: usize) -> bool {
    x == 0
        || x == x_max - 1
        || (0..y).all(|i| grid[i][x] < grid[y][x])
        || (y + 1..grid.len()).all(|i| grid[i][x] < grid[y][x])
        || (0..x).all(|i| grid[y][i] < grid[y][x])
        || (x + 1..grid.len()).all(|i| grid[y][i] < grid[y][x])
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<u32>> = vec![];

    let lines = input.lines().collect_vec();

    for y in 0..lines.len() {
        let chars = lines[y].chars().collect_vec();
        let mut xaxis: Vec<u32> = vec![];
        for x in 0..chars.len() {
            let num = chars[x]
                .to_string()
                .parse::<u32>()
                .expect("Must be a valid u8");
            xaxis.push(num)
        }
        grid.push(xaxis);
    }

    let mut max = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            let score = calc_score(&grid, x, y);
            if score > max {
                max = score
            }
        }
    }

    Some(max)
}

fn calc_score(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    let up = (0..y)
        .rev()
        .take_while(|&i| grid[i][x] < grid[y][x])
        .count() as u32;
    let down = (y + 1..grid.len())
        .take_while(|&i| grid[i][x] < grid[y][x])
        .count() as u32
        + 1;
    let left = (0..x)
        .rev()
        .take_while(|&i| grid[y][i] < grid[y][x])
        .count() as u32;
    let right = (x + 1..grid.len())
        .take_while(|&i| grid[y][i] < grid[y][x])
        .count() as u32;

    return up * down * left * right;
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
        assert_eq!(part_one(&input).unwrap(), 21);
    }
    #[test]
    fn test_part_two() {
        let input = read_example();
        assert_eq!(part_two(&input).unwrap(), 8);
    }
}
