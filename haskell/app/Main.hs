{-# LANGUAGE ViewPatterns #-}

module Main where

import Data.List (stripPrefix)
import qualified Data.Map as M
import Data.Maybe (fromJust, fromMaybe)
import System.Environment (getArgs)

-- * Day 3

-- ** First part

bits2int :: [Bool] -> Int
bits2int bs = go 0 1 (reverse bs)
  where
    go acc pow (True:bs) = go (acc + pow) (pow * 2) bs
    go acc pow (False:bs) = go acc (pow * 2) bs
    go acc _ _ = acc

day3a' :: [String] -> Int
day3a' bs = gamma * epsilon
  where
    bits :: [Bool]
    bits = toBit <$> foldr1 (zipWith (+)) (fmap asBit <$> bs)
    gamma = bits2int $ bits
    epsilon = bits2int (not <$> bits)
    asBit '0' = -1
    asBit '1' = 1
    asBit _ = error "No parse (day3a' asBit)"
    toBit i | i < 0 = False
            | i > 0 = True
            | otherwise = error "No parse (day3a' toBit)"

-- ** Second part

day3b' :: [String] -> Int
day3b' _ = oxy * co2
  where
    oxy = 0
    co2 = 0

-- * Day 2

-- ** First part

data Direction = Forward | Up | Down
  deriving (Show)

data Command = Command Direction Int
  deriving (Show)

readCommand (stripPrefix "forward " -> Just x) = Command Forward (read x)
readCommand (stripPrefix "up " -> Just x) = Command Up (read x)
readCommand (stripPrefix "down " -> Just x) = Command Down (read x)
readCommand x = error $ "No parse (readCommand): " ++ x

day2a' :: [Command] -> Int
day2a' cs = mul $ go cs (0, 0)
  where
    go :: [Command] -> (Int, Int) -> (Int, Int)
    go (Command Forward a : cs) (horz, depth) = go cs (horz + a, depth)
    go (Command Up a : cs) (horz, depth) = go cs (horz, depth - a)
    go (Command Down a : cs) (horz, depth) = go cs (horz, depth + a)
    go _ pos = pos
    mul (a, b) = a * b

day2a = do
  input <- getContents
  print $ day2a' (readCommand <$> lines input)

-- ** Second part

day2b' :: [Command] -> Int
day2b' cs = mul $ go cs (0, 0, 0)
  where
    go :: [Command] -> (Int, Int, Int) -> (Int, Int, Int)
    go (Command Forward a : cs) (horz, depth, aim) = go cs (horz + a, depth+(aim*a), aim)
    go (Command Up a : cs) (horz, depth, aim) = go cs (horz, depth, aim - a)
    go (Command Down a : cs) (horz, depth, aim) = go cs (horz, depth, aim + a)
    go _ pos = pos
    mul (a, b, _) = a * b

day2b = withInputMap readCommand day2b'

-- * Day 1

-- ** Part A

type State = (Int, Int)

day1 :: (Int -> Int -> Bool) -> Int -> [Int] -> Int
day1 cond init xs = fst $ foldr f (0, init) xs
  where
    f item (count, prev)
      | cond item prev = (count + 1, item)
      | otherwise = (count, item)

day1a :: IO ()
day1a = do
  input <- getInput
  print $ day1 (<) (minBound :: Int) input

-- ** Part B

day1b' :: [Int] -> [Int]
day1b' xs = go xs []
  where
    go :: [Int] -> [Int] -> [Int]
    go (a : b : c : xs) acc = go (b : c : xs) (a + b + c : acc)
    go _ acc = acc

day1b :: IO ()
day1b = do
  input <- getInput
  print $ day1 (>) (maxBound :: Int) (day1b' input)

-- putStrLn . show . day1b' $ input

-- * Utilities

getInput :: (Read a) => IO [a]
getInput = do
  fmap read . lines <$> getContents

withInput :: (Show b) => ([String] -> b) -> IO ()
withInput = withInputMap id

withInputMap :: (Show b) => (String -> a) -> ([a] -> b) -> IO ()
withInputMap f f2 = do
  input <- getContents
  print . f2 $ f <$> lines input

-- * Main(s)

mains :: M.Map String (IO ())
mains =
  M.fromList
    [ ("day1a", day1a),
      ("day1b", day1b),
      ("day2a", day2a),
      ("day2b", day2b),
      ("day3a", withInput day3a'),
      ("day3b", withInput day3b')
    ]

usage = do
  putStrLn "Usage: aoc2021 PROGRAM"
  putStr "Where PROGRAM is one of: "
  print $ M.keys mains

getMain :: [String] -> IO ()
getMain [] = usage
getMain (x : _) = fromMaybe usage (M.lookup x mains)

main :: IO ()
main = do
  args <- getArgs
  getMain args
