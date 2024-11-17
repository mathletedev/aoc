use itertools::Itertools;

use crate::{
    solution::{Part, Solution},
    utils::Vector2i,
};

fn gen_prefix(vec: &[bool], time: i64) -> Vec<i64> {
    let mut prefix = vec![0; vec.len() + 1];
    for (i, expanded) in vec.iter().enumerate() {
        prefix[i + 1] = prefix[i] + 1;
        if *expanded {
            prefix[i + 1] += time;
        }
    }

    prefix
}

fn solve(input: &str, time: i64) -> i64 {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();

    let galaxies = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(j, _)| Vector2i::new(j as i32, i as i32))
        })
        .collect_vec();

    let expanded_rows = (0..rows)
        .map(|i| !galaxies.iter().any(|g| g.y == i as i32))
        .collect_vec();
    let expanded_cols = (0..cols)
        .map(|j| !galaxies.iter().any(|g| g.x == j as i32))
        .collect_vec();

    let x_prefix = gen_prefix(&expanded_cols, time);
    let y_prefix = gen_prefix(&expanded_rows, time);

    let dist = |a: Vector2i, b: Vector2i| -> i64 {
        (x_prefix[a.x as usize + 1] - x_prefix[b.x as usize + 1]).abs()
            + (y_prefix[a.y as usize + 1] - y_prefix[b.y as usize + 1]).abs()
    };

    galaxies
        .iter()
        .enumerate()
        .map(|(i, galaxy)| {
            galaxies
                .iter()
                .enumerate()
                .filter(|(j, _)| *j != i)
                .map(|(_, other)| dist(galaxy.clone(), other.clone()))
                .sum::<i64>()
        })
        .sum::<i64>()
        / 2
}

const PART1: Part = |input| solve(input, 1).to_string();
const PART2: Part = |input| solve(input, 1000000 - 1).to_string();

pub const SOLUTION: Solution = Solution {
    part1: Some(PART1),
    part2: Some(PART2),
};
