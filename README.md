# Forth Compiler

Implemented in Rust. Compiles Forth to Web Assembly and then runs it.

**Reason for Web Assembly**: WASM is stack-based, it enforces that function stack is empty at the end of function. 
So we can use this builtin stack as Forth's *return stack*, and a seperate global variable list as *data stack*.

## Installation

- TODO

## Useful Commands

- TODO

## PureScript Resources

- TODO
