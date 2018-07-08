/*!
# MDCCC: Markdown Compiler-Compiler-Compiler

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
 */

#![doc(html_root_url = "https://docs.rs/mdccc/0.1.1")]
#![deny(missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces, unused_qualifications, missing_docs)]
#![cfg_attr(feature = "cargo-clippy",
            allow(single_match, match_same_arms, match_ref_pats,
                  clone_on_ref_ptr, needless_pass_by_value))]
#![cfg_attr(feature = "cargo-clippy",
            deny(wrong_pub_self_convention, used_underscore_binding,
                 stutter, similar_names, pub_enum_variant_names,
                 missing_docs_in_private_items,
                 non_ascii_literal, unicode_not_nfc,
                 result_unwrap_used, option_unwrap_used,
                 option_map_unwrap_or_else, option_map_unwrap_or, filter_map,
                 shadow_unrelated, shadow_reuse, shadow_same,
                 int_plus_one, string_add_assign, if_not_else,
                 invalid_upcast_comparisons,
                 cast_precision_loss, cast_lossless,
                 cast_possible_wrap, cast_possible_truncation,
                 mutex_integer, mut_mut, items_after_statements,
                 print_stdout, mem_forget, maybe_infinite_iter))]

extern crate pulldown_cmark;

pub mod error;
pub mod latex;
pub mod escape;
