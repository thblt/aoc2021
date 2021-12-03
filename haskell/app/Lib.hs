module Lib (getInput) where

getInput :: (Read a) => IO [a]
getInput = do
  raw <- getContents
  return $ fmap read (lines raw)
