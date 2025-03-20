module Optimizer (optimize) where

import Prelude

import Parser (IR)

optimize :: IR -> IR
optimize ir = ir