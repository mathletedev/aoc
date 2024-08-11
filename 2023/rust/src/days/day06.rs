use itertools::Itertools;

use crate::solution::{Part, Solution};

const PART1: Part = |input| {
    let (times, distances) = input
        .lines()
        .map(|line| {
            line.split_once(':')
                .unwrap()
                .1
                .trim()
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect_tuple()
        .unwrap();

    times
        .iter()
        .enumerate()
        .map(|(i, &t)| {
            // check which hold times will beat the record
            (1..t).filter(|x| (x * (t - x) > distances[i])).count()
        })
        .product::<usize>()
        .to_string()
};

// PERF: brute force solution
const PART2: Part = |input| {
    let (time, distance) = input
        .lines()
        .map(|line| {
            line.split_once(':')
                .unwrap()
                .1
                .chars()
                .filter(|c| *c != ' ')
                .join("")
                .parse::<u128>()
                .unwrap()
        })
        .collect_tuple()
        .unwrap();

    (1..time)
        .filter(|x| (x * (time - x) > distance))
        .count()
        .to_string()
};

pub const SOLUTION: Solution = Solution {
    part1: Some(PART1),
    part2: Some(PART2),
};
