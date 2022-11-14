-----------
-- DAY 3 --
-----------

import Data.Bits (bit, (.&.))

day3a :: [Int] -> Int
day3a bs = gamma * epsilon
  where
    bits :: [Bool]
    bits = toBit <$> foldr1 (zipWith (+)) (fmap asBit <$> bs)
    gamma = bitsToInt $ bits
    epsilon = bitsToInt (not <$> bits)
    asBit '0' = -1
    asBit '1' = 1
    asBit _ = error "No parse (day3a' asBit)"
    toBit i | i < 0 = False
            | i > 0 = True
            | otherwise = error "No parse (day3a toBit)"

class Bitty b where
  asBit :: b -> Bool

instance Bitty Bool where
  asBit = id

instance Bitty Char where
  asBit '0' = False
  asBit '1' = True
  asBit _ = error "No parse (asBit for Char)"

-- Read a bit String into an Int.
bitsToInt :: (Bitty b) => [b] -> Int
bitsToInt bs = f 0 (length bs - 1) (asBit <$> bs) where
  f acc (-1) _ = acc
  f acc bit (False : bs) = f acc (bit - 1) bs
  f acc bit (True : bs) = f (acc + 2 ^ bit) (bit - 1) bs
  f _ _ _ = error "readBits: no parse"

getBit :: Int -> Int -> Bool
getBit b n = (n .&. bit b) /= 0

count :: (a -> Bool) -> [a] -> (Int,Int)
count pred ls = f ls (0,0)
  where
    f [] acc = acc
    f (l:ls) (t,f) | pred l = (t+1,f)
                   | otherwise = (t, f+1)

pick :: (Int, Int) -> Bool
pick (a,b) = a >= b
