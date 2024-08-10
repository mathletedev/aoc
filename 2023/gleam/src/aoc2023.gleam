import birl
import birl/duration.{MicroSecond}
import days/day01
import gleam/erlang
import gleam/int
import gleam/io
import gleam/option.{Some}
import gleam/string
import simplifile

pub fn main() {
  let assert Ok(day) = erlang.get_line("Enter day: ")
  let assert Ok(day) =
    day
    |> string.trim
    |> int.parse

  let assert Ok(part) = erlang.get_line("Enter part: ")
  let assert Ok(part) =
    part
    |> string.trim
    |> int.parse

  let input = case
    simplifile.read(
      "../input/day"
      <> day
      |> string.inspect
      |> string.pad_left(to: 2, with: "0")
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

  let start = birl.now()

  let ans = solution(input)

  let elapsed = birl.difference(birl.now(), start)
  let elapsed_ms =
    int.to_float(duration.blur_to(elapsed, MicroSecond)) /. 1000.0

  io.println("")
  io.println(ans)
  io.println("")
  io.println("Executed in " <> string.inspect(elapsed_ms) <> "ms")
}
