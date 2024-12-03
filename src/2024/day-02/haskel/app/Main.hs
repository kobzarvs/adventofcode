module Main (main) where

import Lib
import System.Environment (getArgs)
import Prelude
--import Debug.Trace


try_and_drop :: Int -> [Int] -> Bool
try_and_drop x xs =
  let (left, right) = splitAt x xs
      new_list = left ++ (drop 1 right)
   in is_safe_report new_list


find_one_dropped :: [Int] -> Bool
find_one_dropped xs = or $ map (\x -> try_and_drop x xs) [0 .. (length xs) - 1]


drop_one_and_check :: Bool -> [Int] -> Bool
drop_one_and_check strict xs =
    let safe = is_safe_report xs
    in case safe of
        True -> True
        False ->
            if strict
                then False
                else find_one_dropped xs


check_dir :: [Int] -> Bool
check_dir xs = 
    -- trace ("xs: " ++ show xs ++ ", directions: " ++ show directions) $
    all (== head directions) directions
  where
    directions = zipWith (\a b -> signum (a - b)) xs (tail xs)


check_pairs_acc :: [Int] -> Bool
check_pairs_acc xs = all checkAndDistance (zip xs (tail xs))
  where
    checkAndDistance (a, b) =
        abs (b - a) >= 1 && abs (b - a) <= 3


is_safe_report :: [Int] -> Bool
is_safe_report xs = check_pairs_acc(xs) && check_dir(xs)


filter_safe_reports :: [[Int]] -> Bool -> [[Int]]
filter_safe_reports xss strict = filter(\xs -> drop_one_and_check strict xs) xss

main :: IO ()
main = do
    filepath <- getArgs
    putStrLn $ "file: " ++ show(filepath !! 0)

    input <- readFile(filepath !! 0)
    let reports = map strToIntArray . map words $ lines input

    putStrLn $ "Part I: " ++ show(length $ filter_safe_reports reports True)
    putStrLn $ "Part II: " ++ show(length $ filter_safe_reports reports False)
