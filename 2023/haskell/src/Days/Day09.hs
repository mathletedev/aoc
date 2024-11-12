module Days.Day09 where

import Data.List.Split

-- | Calculate the difference between adjacent elements in a list
diff :: (Num a) => [a] -> [a]
diff [] = []
diff (x : xs)
  | null xs = []
  | otherwise = (head xs - x) : diff xs

-- | Extrapolate the next number in a sequence
extrapolate :: [Int] -> Int
extrapolate xs
  | all (== 0) xs = head xs
  | otherwise = last xs + extrapolate (diff xs)

part1 :: String -> String
part1 =
  show
    . sum
    . fmap (extrapolate . map (read :: String -> Int) . splitOn " ")
    . lines

part2 :: String -> String
part2 =
  show
    . sum
    . fmap (extrapolate . reverse . map (read :: String -> Int) . splitOn " ")
    . lines
