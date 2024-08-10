import days/day01
import gleam/erlang
import gleam/int
import gleam/io
import gleam/option.{Some}
import gleam/string
import simplifile

pub fn main() {
  let assert Ok(day) = erlang.get_line("Enter day: ")
  let assert Ok(day) = day |> string.trim |> int.parse

  let assert Ok(part) = erlang.get_line("Enter part: ")
  let assert Ok(part) = part |> string.trim |> int.parse

  let input = case
    simplifile.read(
      "../input/day"
      <> day |> string.inspect |> string.pad_left(to: 2, with: "0")
      <> ".txt",
    )
  {
    Ok(input) -> input
    Error(_) -> panic as "Input file not found"
  }

  let solution = case day {
    1 -> day01.solution
    _ -> panic
  }

  let assert Some(solution) = case part {
    1 -> solution.part1
    2 -> solution.part2
    _ -> panic
  }

  // TODO: implement benchmarking

  let ans = solution(input)

  io.println(ans)
}
