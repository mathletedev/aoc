use std::collections::HashMap;

use itertools::Itertools;

use crate::solution::{Part, Solution};

const PART1: Part = |input| {
    let (first, rest) = input.split_once("\n\n").unwrap();
    let instructions = first.chars().map(|c| c == 'R').collect_vec();

    let mut network = HashMap::new();
    rest.lines().for_each(|line| {
        let (node, left, right) = line
            .split(|c: char| !c.is_ascii_uppercase())
            .filter(|x| !x.is_empty())
            .collect_tuple()
            .unwrap();

        network.insert(node, (left, right));
    });

    let mut curr_node = "AAA";
    let mut count = 0;
    while curr_node != "ZZZ" {
        let (left, right) = network[curr_node];
        curr_node = match instructions[count % instructions.len()] {
            true => right,
            false => left,
        };

        count += 1;
    }

    count.to_string()
};

pub const SOLUTION: Solution = Solution {
    part1: Some(PART1),
    part2: None,
};
