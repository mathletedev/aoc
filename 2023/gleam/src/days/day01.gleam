import gleam/int
import gleam/list
import gleam/option.{None, Some}
import gleam/result
import gleam/string
import solution.{Solution}

/// converts a character to its ascii value
fn ctoa(c: String) -> Int {
  c
  |> string.to_utf_codepoints
  |> list.map(fn(v) { string.utf_codepoint_to_int(v) })
  |> list.first
  |> result.unwrap(0)
}

fn is_digit(c: String) -> Bool {
  ctoa(c) >= 48 && ctoa(c) <= 57
}

fn find_digit(line: String) -> Int {
  line
  |> string.to_graphemes
  |> list.find(fn(x) { is_digit(x) })
  |> result.unwrap("0")
  |> int.parse
  |> result.unwrap(0)
}

fn part1(input: String) -> String {
  input
  |> string.split("\n")
  |> list.map(fn(line) {
    10
    * find_digit(line)
    + {
      line
      |> string.reverse
      |> find_digit
    }
  })
  |> int.sum
  |> int.to_string
}

pub const solution = Solution(part1: Some(part1), part2: None)
