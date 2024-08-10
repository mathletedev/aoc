use itertools::Itertools;

use crate::solution::{Part, Solution};

const PART1: Part = |input| input.split("\n\n").join("\n\n\n");

pub const SOLUTION: Solution = Solution {
    part1: Some(PART1),
    part2: None,
};
