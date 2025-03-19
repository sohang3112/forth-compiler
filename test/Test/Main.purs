module Test.Main where

import Prelude

import Effect (Effect)
import Effect.Console (log)
import Test.Spec (Spec, it)
import Test.Spec.Assertions (shouldEqual)
import Test.Spec.Reporter (consoleReporter)
import Test.Spec.Runner.Node (runSpecAndExitProcess)

main :: Effect Unit
main = runSpecAndExitProcess [consoleReporter] spec

spec :: Spec Unit
spec = it "adds 1 and 1" do
  (1 + 1) `shouldEqual` 2
