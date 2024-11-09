module Days.Day09 where

import Data.List.Split

allEqual :: (Eq a) => [a] -> Bool
allEqual xs = all (== head xs) (tail xs)

diff :: (Num a) => [a] -> [a]
diff [] = []
diff (x : xs)
  | null xs = []
  | otherwise = (head xs - x) : diff xs

extrapolate :: [Int] -> Int
extrapolate xs
  | allEqual xs = head xs
  | otherwise = last xs + extrapolate (diff xs)

part1 :: String -> String
part1 input =
  show
    . sum
    . map (extrapolate . map (read :: String -> Int) . splitOn " ")
    $ lines input
