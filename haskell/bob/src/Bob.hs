{-# OPTIONS_GHC -Werror -Wall #-}

module Bob (responseFor) where

import Data.List (isSuffixOf, dropWhileEnd)
import Data.Char (isSpace, isAlpha, toUpper)

responseFor :: String -> String
responseFor xs
  | all isSpace xs = "Fine. Be that way!"
  | any isAlpha xs && all (\c -> toUpper c == c) xs = "Whoa, chill out!"
  | "?" `isSuffixOf` dropWhileEnd isSpace xs = "Sure."
  | otherwise = "Whatever."
