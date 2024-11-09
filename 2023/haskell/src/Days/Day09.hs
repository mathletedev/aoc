module Days.Day09 where

import Data.List.Split

-- | Check if all elements in a list are equal
allEqual :: (Eq a) => [a] -> Bool
allEqual xs = all (== head xs) (tail xs)

-- | Calculate the difference between adjacent elements in a list
diff :: (Num a) => [a] -> [a]
diff [] = []
diff (x : xs)
  | null xs = []
  | otherwise = (head xs - x) : diff xs

-- | Extrapolate the next number in a sequence
extrapolate :: [Int] -> Int
extrapolate xs
  | allEqual xs = head xs
  | otherwise = last xs + extrapolate (diff xs)

part1 :: String -> String
part1 =
  show
    . sum
    . map (extrapolate . map (read :: String -> Int) . splitOn " ")
    . lines

part2 :: String -> String
part2 =
  show
    . sum
    . map (extrapolate . reverse . map (read :: String -> Int) . splitOn " ")
    . lines
