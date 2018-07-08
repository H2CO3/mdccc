# MDCCC: Markdown Compiler-Compiler-Compiler

[![MDCCC on crates.io](https://img.shields.io/crates/v/mdccc.svg)](https://crates.io/crates/mdccc)
[![MDCCC on docs.rs](https://docs.rs/mdccc/badge.svg)](https://docs.rs/mdccc)
[![MDCCC Download](https://img.shields.io/crates/d/mdccc.svg)](https://crates.io/crates/mdccc)
[![MDCCC License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/H2CO3/mdccc/blob/master/LICENSE.txt)
[![Lines of Code](https://tokei.rs/b1/github/H2CO3/mdccc)](https://github.com/Aaronepower/tokei)
[![Twitter](https://img.shields.io/badge/twitter-@H2CO3_iOS-blue.svg?style=flat&colorB=64A5DE&label=Twitter)](http://twitter.com/H2CO3_iOS)

MDCCC is a Markdown to LaTeX renderer. It's not a compiler compiler compiler;
it's not even a compiler compiler, but I find the expression oddly satisfying.
It's also the Roman numeral for 1800.

## Usage of the library

The basic idea: there's a `LaTeXIter` type, which is an iterator adaptor. It
consumes an `Iterator` over Markdown `Event`s (as defined by the
`pulldown_cmark` crate), and outputs a stream of LaTeX string fragments.
Hopefully its documentation is good enough so that you'll be able to figure
out the rest. See `src/bin/mdccc.rs` for the *very simple* example usage.

## Usage of the CLI tool

The crate comes with a command-line utility, `mdccc`, which is very easy to
use. In the spirit of Unix, all it does is it reads Markdown from `stdin`
and spits out LaTeX to `stdout` like a filter. You can therefore do something
like this in order to convert a Markdown file to a PDF:

    mdccc < input.md > output.tex
    pdflatex output.tex
