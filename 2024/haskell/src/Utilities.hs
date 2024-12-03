module Utilities where

import Data.List.Split

parseInts :: String -> String -> [Int]
parseInts sep = map read . splitOn sep
