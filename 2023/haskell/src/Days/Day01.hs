module Days.Day01 where

import Data.Char
import Data.List
import Data.Maybe

part1 :: String -> String
part1 input =
  show
    $ sum
    $ map
      ( \line ->
          digitToInt (fromMaybe '0' (find isDigit line)) * 10
            + digitToInt (fromMaybe '0' (find isDigit $ reverse line))
      )
    $ lines input
