module Interpreter where

import Prelude

import Parser (IR)

type Stack = List Int
type Result = Result { dataStack :: Stack, returnStack :: Stack }

interpret :: IR -> 