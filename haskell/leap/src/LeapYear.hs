module LeapYear (isLeapYear) where

isLeapYear :: Integer -> Bool
isLeapYear year = div_by 4 && not (div_by 100) || div_by 400
  where
    div_by n = year `rem` n == 0
