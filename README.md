# Forth Compiler

Implemented in PureScript. Compiles Forth to JavaScript and then runs it.

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

### Spago (PureScript package manager) Resources

- [`spago@next`](https://github.com/purescript/spago): latest Spago version.
