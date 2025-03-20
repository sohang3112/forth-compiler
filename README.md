# Forth Compiler

Implemented in PureScript. Compiles Forth to Web Assembly and then runs it.

**Reason for Web Assembly**: WASM is stack-based, it enforces that function stack is empty at the end of function. 
So we can use this builtin stack as Forth's *return stack*, and a seperate global variable list as *data stack*.

Tested on:

- [ ] NodeJS
- [ ] Browsers: Firefox, Chrome, Edge

## Installation

Followed [Setting Up guide for PureScript](https://github.com/purescript/documentation/blob/master/guides/Getting-Started.md):

```bash
$ npm install -g purescript & purs --version      # install PureScript
$ npm install -g spago@next & spago --version          # install Spago, PureSript's package manager
$ spago run     # or `spago test` or `spago build`
```

## Useful Commands

- Automatically rebuild on file changes: `spago build --watch --clear-screen`
- Import & run in `spago repl` instead of CLI:

```purescript
> import Main
> main
42
```

## PureScript Resources

- [Official Book](https://book.purescript.org)
- https://jordanmartinez.github.io/purescript-jordans-reference-site/

### Communities

- [r/purescript](https://www.reddit.com/r/purescript/)

### Packages

- [`spago@next` package manager](https://github.com/purescript/spago): latest version
- [Pursuit package docs](https://pursuit.purescript.org/)
