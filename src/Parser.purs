module Parser (IR(..), parse) where

import Prelude

import Data.Either (Either(..))
import Effect (Effect)
import Effect.Console (log)
import Node.FS.Async as FS
import Parsing (Parser, ParseError, runParser)
import Parsing.Combinators (sepBy1, many, many1)
import Parsing.String (anyChar, satisfy, rest)
import Parsing.String.Basic (alphaNum, space, skipSpaces)

newtype IR = IR String

program :: Parser String IR
program = IR <$> rest

parse :: String -> Either ParseError IR
parse = flip runParser program

