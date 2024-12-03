module Lib
    ( strToIntArray
    ) where

strToIntArray :: [String] -> [Int]
strToIntArray [] = []
strToIntArray ls = map read ls
