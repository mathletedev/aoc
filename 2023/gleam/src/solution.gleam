import gleam/option.{type Option}

type Part =
  fn(String) -> String

pub type Solution {
  Solution(part1: Option(Part), part2: Option(Part))
}
