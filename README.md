# Quickfind

As a hater of IDEs, I (grudgingly) accept that it's nice to be able to press a button in my editor
and visit the definition of a class or function. This project is a lightweight util which uses
[ag](https://github.com/ggreer/the_silver_searcher) and some pattern searches to look up symbols
in my code in the local project, and report results in a way that makes it easy to visit the
destinations.

It's also able to find existing imports for a symbol and write an import for that symbol.

It's not perfect as it relies on unique names and isn't context-aware as it uses pattern matching.
It's also only able to find definitions in the local project, and copy imports which have already
been used somewhere else. However this works rapidly for the majority of cases and I prefer the
approach to using an IDE or a heavyweight tool like [Metals](https://scalameta.org/metals/).

This is used in conjunction with some delegating vim functions in order to power some lookup and
import-writing features. It supports a few languages I use, including Scala, Python, Rust, and Go.
I'll be adding better multi-language support as I go.

This is a Rust rewrite of the original I wrote in python a few years ago:
[quickfind](https://github.com/giftig/quickfind).

## Installation

Install from source using `make && make install`.

## Runtime requirements

- [ag](https://github.com/ggreer/the_silver_searcher) `>= 2.2.0`
