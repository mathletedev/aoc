module Days.Day01 where

import Data.Char
import Data.List
import Data.Maybe

-- | Find the first digit in a string and convert it to an int
findDigit :: String -> Int
findDigit = digitToInt . fromMaybe '0' . find isDigit

part1 :: String -> String
part1 =
  show
    . sum
    . fmap (\line -> findDigit line * 10 + findDigit (reverse line))
    . lines
