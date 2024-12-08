use itertools::Itertools;

use crate::{
    solution::{Part, Solution},
    utils::Vector2i,
};

const PART1: Part = |input| {
    let mut map = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut pos = Vector2i::ZERO;
    let mut dir = Vector2i::ZERO;
    for (i, row) in map.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if !matches!(c, '^' | 'v' | '>' | '<') {
                continue;
            }

            pos = Vector2i::new(j as i32, i as i32);

            match c {
                '^' => dir = Vector2i::UP,
                '>' => dir = Vector2i::RIGHT,
                'v' => dir = Vector2i::DOWN,
                '<' => dir = Vector2i::LEFT,
                _ => unreachable!(),
            }

            break;
        }

        if dir != Vector2i::ZERO {
            break;
        }
    }

    while pos.x >= 0 && pos.x < map[0].len() as i32 && pos.y >= 0 && pos.y < map.len() as i32 {
        if map[pos.y as usize][pos.x as usize] == '#' {
            pos -= dir.clone();
            dir = match dir {
                Vector2i::UP => Vector2i::RIGHT,
                Vector2i::RIGHT => Vector2i::DOWN,
                Vector2i::DOWN => Vector2i::LEFT,
                Vector2i::LEFT => Vector2i::UP,
                _ => unreachable!(),
            }
        } else {
            map[pos.y as usize][pos.x as usize] = 'X';
        }

        pos += dir.clone();
    }

    map.iter()
        .map(|row| row.iter().filter(|c| **c == 'X').count() as u32)
        .sum::<u32>()
        .to_string()
};

pub const SOLUTION: Solution = Solution {
    part1: Some(PART1),
    part2: None,
};
