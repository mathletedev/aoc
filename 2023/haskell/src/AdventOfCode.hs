module AdventOfCode (solve) where

import Days.Day01

solve :: Int -> Int -> String -> String
solve day part
  | day == 1 && part == 1 = Days.Day01.part1
  | otherwise = error
