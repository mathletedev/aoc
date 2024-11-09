module AdventOfCode (solve) where

import Days.Day01
import Days.Day09

solve :: Int -> Int -> String -> String
solve day part
  | day == 1 && part == 1 = Days.Day01.part1
  | day == 9 && part == 1 = Days.Day09.part1
  | otherwise = error
