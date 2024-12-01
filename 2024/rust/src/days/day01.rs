use std::collections::HashMap;

use itertools::Itertools;

use crate::solution::{Part, Solution};

const PART1: Part = |input| {
    let mut left = vec![];
    let mut right = vec![];

    for line in input.lines() {
        let (a, b) = line
            .split("   ")
            .map(|s| s.parse::<u32>().unwrap())
            .collect_tuple()
            .unwrap();

        left.push(a);
        right.push(b);
    }

    left.sort();
    right.sort();

    (0..left.len())
        .map(|i| left[i].abs_diff(right[i]))
        .sum::<u32>()
        .to_string()
};

const PART2: Part = |input| {
    let mut frequencies = HashMap::new();

    let mut left = vec![];
    let mut right = vec![];

    for line in input.lines() {
        let (a, b) = line
            .split("   ")
            .map(|s| s.parse::<u32>().unwrap())
            .collect_tuple()
            .unwrap();

        left.push(a);
        right.push(b);

        frequencies.insert(a, 0);
    }

    for x in right {
        frequencies.entry(x).and_modify(|f| *f += 1);
    }

    left.iter()
        .map(|x| x * frequencies[x])
        .sum::<u32>()
        .to_string()
};

pub const SOLUTION: Solution = Solution {
    part1: Some(PART1),
    part2: Some(PART2),
};
