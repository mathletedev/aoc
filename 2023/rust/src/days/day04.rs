use itertools::Itertools;

use crate::solution::{Part, Solution};

const PART1: Part = |input| {
    input
        .lines()
        .map(|line| {
            let data = line.split_once(':').unwrap().1;

            let (winning, have) = data
                .split('|')
                .map(|d| {
                    d.split(' ')
                        .filter_map(|n| match n.parse::<i32>() {
                            Ok(n) => Some(n),
                            Err(_) => None,
                        })
                        .collect::<Vec<i32>>()
                })
                .collect_tuple()
                .unwrap();

            // count number of winning numbers in have
            let num_winning = have.iter().filter(|n| winning.contains(n)).count();

            if num_winning > 0 {
                1 << (num_winning - 1)
            } else {
                0
            }
        })
        .sum::<i32>()
        .to_string()
};

const PART2: Part = |input| {
    const NUM_CARDS: usize = 201;

    // use push dp
    let mut dp = vec![1; NUM_CARDS];

    input.lines().enumerate().for_each(|(i, line)| {
        let data = line.split_once(':').unwrap().1;

        let (winning, have) = data
            .split('|')
            .map(|d| {
                d.split(' ')
                    .filter_map(|n| match n.parse::<i32>() {
                        Ok(n) => Some(n),
                        Err(_) => None,
                    })
                    .collect::<Vec<i32>>()
            })
            .collect_tuple()
            .unwrap();

        let num_winning = have.iter().filter(|n| winning.contains(n)).count();

        // for the next num_winning cards, add dp[i] copies
        (1..=num_winning).for_each(|j| {
            if (i + j) < NUM_CARDS {
                dp[i + j] += dp[i];
            }
        })
    });

    dp.iter().sum::<i32>().to_string()
};

// NOTE: try implementing part 2 without mutability, using recursion

pub const SOLUTION: Solution = Solution {
    part1: Some(PART1),
    part2: Some(PART2),
};
