module Main where

import qualified AdventOfCode (solve)
import System.IO

padLeft :: Int -> Char -> String -> String
padLeft n c s
  | length s >= n = s
  | otherwise = padLeft n c (c : s)

main :: IO ()
main =
  do
    putStr "Enter day: "
    hFlush stdout
    line1 <- getLine
    let day = read line1 :: Int

    putStr "Enter part: "
    hFlush stdout
    line2 <- getLine
    let part = read line2 :: Int

    input <- readFile $ "../input/day" ++ padLeft 2 '0' (show day) ++ ".txt"

    putStrLn $ AdventOfCode.solve day part input
