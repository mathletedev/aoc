module Days.Day01 where

import Data.List
import Utilities

dists :: [[Int]] -> [Int]
dists xs = zipWith (-) (head xs) (last xs)

part1 :: String -> String
part1 =
  show
    . sum
    . fmap abs
    . dists
    . fmap sort
    . transpose
    . fmap (parseInts "   ")
    . lines
