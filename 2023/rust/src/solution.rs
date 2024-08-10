pub type Part = fn(&str) -> String;

pub struct Solution {
    pub part1: Option<Part>,
    pub part2: Option<Part>,
}
