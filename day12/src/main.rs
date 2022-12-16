use std::{borrow::BorrowMut, cmp::Ordering::Equal, collections::HashMap};

use helpers::{print_day, print_solution, read_input};

mod helpers;

type Position = (usize, usize);

#[derive(Clone)]
struct Node {
    elevation: usize,
    pos: (usize, usize),
    distance: f32,
}

fn shortest_path(
    start: Position,
    end: Position,
    grid: &mut HashMap<Position, Node>,
) -> Option<f32> {
    // Set starting distance to 0
    grid.get_mut(&start).unwrap().distance = 0.;
    let mut next = start;

    loop {
        let current = grid.remove(&next).unwrap();
        let (y, x) = current.pos;

        if (y, x) == end {
            return Some(current.distance);
        }

        // Update distances of neighbors if they are valid positions
        neighbor_distance(grid, &current, (y, x + 1));
        neighbor_distance(grid, &current, (y + 1, x));
        if x > 0 {
            neighbor_distance(grid, &current, (y, x - 1));
        }
        if y > 0 {
            neighbor_distance(grid, &current, (y - 1, x));
        }

        // Set the smallest distance as the next position to check
        if let Some(min) = grid
            .iter()
            .min_by(|a, b| a.1.distance.partial_cmp(&b.1.distance).unwrap_or(Equal))
            .map(|(_, n)| n.pos)
        {
            next = min;
        }
    }
}

fn neighbor_distance(grid: &mut HashMap<Position, Node>, current: &Node, pos: Position) {
    if let Some(n) = grid.get_mut(&(pos)) {
        if n.elevation <= current.elevation + 1 {
            n.distance = n.distance.min(current.distance + 1.);
        }
    }
}

fn get_grid(input: &str) -> (Position, Position, HashMap<Position, Node>, Vec<Position>) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut grid: HashMap<Position, Node> = HashMap::new();
    let mut a_pos: Vec<Position> = vec![];

    for (y, line) in input.lines().enumerate() {
        for (x, mut char) in line.chars().enumerate() {
            let (pos, distance) = ((y, x), f32::INFINITY);
            if char == 'S' {
                start = pos;
                char = 'a';
            }
            if char == 'E' {
                end = pos;
                char = 'z';
            }

            if char == 'a' {
                a_pos.push(pos);
            }

            let elevation = (char as u8 - 'a' as u8) as usize;
            grid.insert(
                pos,
                Node {
                    pos,
                    distance,
                    elevation,
                },
            );
        }
    }

    (start, end, grid, a_pos)
}

pub fn part_one(input: &str) -> Option<f32> {
    let (start, end, mut grid, _) = get_grid(input);
    shortest_path(start, end, &mut grid)
}

pub fn part_two(input: &str) -> Option<f32> {
    let (_start, end, grid, a_pos) = get_grid(input);
    // Very slow but whatever
    a_pos
        .iter()
        .map(|a| shortest_path(*a, end, grid.clone().borrow_mut()))
        .min_by(|a, b| a.partial_cmp(&b).unwrap_or(Equal))
        .unwrap()
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
        assert_eq!(part_one(&input).unwrap(), 31 as f32);
    }
    #[test]
    fn test_part_two() {
        let input = read_example();
        assert_eq!(part_two(&input), None);
    }
}
