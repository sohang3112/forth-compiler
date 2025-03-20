module Main (main) where

import Prelude

import Effect (Effect)
import Effect.Console (log)

import Parser (IR, parse)
import Optimizer (optimize)

main :: Effect Unit
main = do
  log "Hello World!"

