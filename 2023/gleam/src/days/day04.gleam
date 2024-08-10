import gleam/int
import gleam/list
import gleam/option.{None, Some}
import gleam/result
import gleam/string
import solution.{Solution}

fn split_into_ints(s: String) -> List(Int) {
  s
  |> string.split(" ")
  |> list.filter(fn(x) { x != "" })
  |> list.map(fn(x) { x |> int.parse |> result.unwrap(0) })
}

fn part1(input: String) -> String {
  input
  |> string.split("\n")
  |> list.map(fn(line) {
    let #(_, data) = line |> string.split_once(":") |> result.unwrap(#("", ""))
    let #(winning, have) =
      data |> string.split_once("|") |> result.unwrap(#("", ""))

    let winning = split_into_ints(winning)
    let have = split_into_ints(have)

    let num_winning =
      have |> list.filter(fn(x) { list.contains(winning, x) }) |> list.length

    case num_winning {
      0 -> 0
      _ -> int.bitwise_shift_left(1, num_winning - 1)
    }
  })
  |> list.reduce(fn(acc, x) { acc + x })
  |> result.unwrap(0)
  |> string.inspect
}

pub const solution = Solution(part1: Some(part1), part2: None)
