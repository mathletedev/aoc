module Days.Day09 where

import Data.List.Split

-- | Extrapolate the next number in a sequence
extrapolate :: [Int] -> Int
extrapolate xs
  | all (== 0) xs = head xs
  | otherwise = last xs + extrapolate (zipWith (-) (tail xs) xs)

part1 :: String -> String
part1 =
  show
    . sum
    . fmap (extrapolate . fmap (read :: String -> Int) . splitOn " ")
    . lines

part2 :: String -> String
part2 =
  show
    . sum
    . fmap (extrapolate . reverse . fmap (read :: String -> Int) . splitOn " ")
    . lines
