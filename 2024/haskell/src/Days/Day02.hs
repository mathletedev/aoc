module Days.Day02 where

import Utilities (parseInts)

safe :: [Int] -> Bool
safe xs = all (\x -> x * head xs > 0) xs && all (\x -> abs x >= 1 && abs x <= 3) xs

diffs :: [Int] -> [Int]
diffs xs = zipWith (-) (tail xs) xs

part2Helper :: [Int] -> Bool
part2Helper xs =
  (safe . diffs) xs
    || any (safe . diffs) [take i xs ++ drop (i + 1) xs | i <- [0 .. length xs - 1]]

part1 :: String -> String
part1 =
  show
    . length
    . filter (safe . diffs . parseInts " ")
    . lines

part2 :: String -> String
part2 =
  show
    . length
    . filter (part2Helper . parseInts " ")
    . lines
