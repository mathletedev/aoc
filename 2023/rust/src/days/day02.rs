use std::{cmp, collections::HashMap};

use crate::solution::{Part, Solution};

const PART1: Part = |input| {
    let possible = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    input
        .lines()
        .enumerate()
        .filter(|(_, line)| {
            // split data into individual colours
            let data = line.split_once(':').unwrap().1;
            let data = data.split(';').collect::<Vec<&str>>();
            let data = data
                .iter()
                .flat_map(|d| d.split(',').collect::<Vec<&str>>());
            let data = data.map(|s| s.trim());

            // if any colour exceeds possible amount, return false
            !data.into_iter().any(|d| {
                let (num, colour) = d.split_once(' ').unwrap();
                let num = num.parse::<u32>().unwrap();

                num > *possible.get(colour).unwrap()
            })
        })
        .map(|(i, _)| (i + 1) as u32)
        .sum::<u32>()
        .to_string()
};

const PART2: Part = |input| {
    input
        .lines()
        .map(|line| {
            let data = line.split_once(':').unwrap().1;
            let data = data.split(';').collect::<Vec<&str>>();
            let data = data
                .iter()
                .flat_map(|d| d.split(',').collect::<Vec<&str>>());
            let data = data.map(|s| s.trim());

            let mins = data
                .into_iter()
                .map(|d| {
                    let (num, colour) = d.split_once(' ').unwrap();
                    let num = num.parse::<u32>().unwrap();

                    // position colour into tuple
                    match colour {
                        "red" => (num, 0, 0),
                        "green" => (0, num, 0),
                        "blue" => (0, 0, num),
                        _ => unreachable!(),
                    }
                })
                // find minimums for each colour
                .fold((0, 0, 0), |acc, (r, g, b)| {
                    (cmp::max(acc.0, r), cmp::max(acc.1, g), cmp::max(acc.2, b))
                });

            mins.0 * mins.1 * mins.2
        })
        .sum::<u32>()
        .to_string()
};

pub const SOLUTION: Solution = Solution {
    part1: Some(PART1),
    part2: Some(PART2),
};
