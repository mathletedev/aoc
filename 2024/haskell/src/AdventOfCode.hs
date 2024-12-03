module AdventOfCode (solve) where

import Days.Day02

solve :: Int -> Int -> String -> String
solve day part
  | day == 2 && part == 1 = Days.Day02.part1
  | day == 2 && part == 2 = Days.Day02.part2
  | otherwise = error
