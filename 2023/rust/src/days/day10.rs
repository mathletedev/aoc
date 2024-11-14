use std::collections::VecDeque;

use itertools::Itertools;

use crate::{
    solution::{Part, Solution},
    utils::Vector2i,
};

const PART1: Part = |input| {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();

    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let mut dists = vec![vec![-1; cols]; rows];

    let mut frontier = VecDeque::new();

    for (y, row) in grid.iter().enumerate() {
        if !frontier.is_empty() {
            break;
        }

        for (x, c) in row.iter().enumerate() {
            if *c == 'S' {
                frontier.push_back((Vector2i::new(x as i32, y as i32), 0));
                break;
            }
        }
    }

    while let Some(curr) = frontier.pop_front() {
        let (pos, dist) = curr;

        if pos.x < 0
            || pos.x as usize >= cols
            || pos.y < 0
            || pos.y as usize >= rows
            || dists[pos.y as usize][pos.x as usize] != -1
        {
            continue;
        }

        dists[pos.y as usize][pos.x as usize] = dist;

        let dirs = match grid[pos.y as usize][pos.x as usize] {
            '|' => vec![Vector2i::UP, Vector2i::DOWN],
            '-' => vec![Vector2i::LEFT, Vector2i::RIGHT],
            'L' => vec![Vector2i::UP, Vector2i::RIGHT],
            'J' => vec![Vector2i::UP, Vector2i::LEFT],
            '7' | 'S' => vec![Vector2i::DOWN, Vector2i::LEFT],
            'F' => vec![Vector2i::DOWN, Vector2i::RIGHT],
            _ => vec![],
        };

        frontier.extend(dirs.iter().map(|dir| (pos.clone() + dir.clone(), dist + 1)));
    }

    dists.iter().flatten().max().unwrap().to_string()
};

pub const SOLUTION: Solution = Solution {
    part1: Some(PART1),
    part2: None,
};
