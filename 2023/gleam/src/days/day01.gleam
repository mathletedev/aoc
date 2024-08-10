import gleam/int
import gleam/list
import gleam/option.{None, Some}
import gleam/order
import gleam/result
import gleam/string
import solution.{Solution}

fn part1(input: String) -> String {
  input
  |> string.split("\n")
  |> list.map(fn(line) {
    10
    * {
      line
      |> string.to_graphemes
      |> list.find(fn(c) {
        let cmp1 = string.compare(c, "0")
        let cmp2 = string.compare(c, "9")

        { cmp1 == order.Eq || cmp1 == order.Gt }
        && { cmp2 == order.Eq || cmp2 == order.Lt }
      })
      |> result.unwrap("0")
      |> int.parse
      |> result.unwrap(0)
    }
    + {
      line
      |> string.reverse
      |> string.to_graphemes
      |> list.find(fn(c) {
        let cmp1 = string.compare(c, "0")
        let cmp2 = string.compare(c, "9")

        { cmp1 == order.Eq || cmp1 == order.Gt }
        && { cmp2 == order.Eq || cmp2 == order.Lt }
      })
      |> result.unwrap("0")
      |> int.parse
      |> result.unwrap(0)
    }
  })
  |> int.sum
  |> int.to_string
}

pub const solution = Solution(part1: Some(part1), part2: None)
