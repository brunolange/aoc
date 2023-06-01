import Data.List

lookAndSays :: String -> String
lookAndSays xs = intercalate "" $ map f $ groupBy (==) xs
  where f grp = show (length grp) ++ [head grp]

main :: IO()
main = print $ (line, length line)
  where line = last $ take 41 $ iterate lookAndSays "1113122113"
