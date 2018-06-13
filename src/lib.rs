//! # MDCCC, a Markdown-to-LaTeX renderer

#![doc(html_root_url = "https://docs.rs/mdccc/0.1.0")]
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

use std::fmt;
use std::io;
use std::borrow::Cow;
use pulldown_cmark::{ Parser, Options, Event };

/// Iterator adapter converting a stream of Markdown events to a stream of LaTeX.
#[derive(Debug, Clone, Copy)]
pub struct LaTeXIter<'a, T: Iterator<Item=Event<'a>>> {
    /// The iterator being wrapped.
    events: T,
}

impl<'a, T: Iterator<Item=Event<'a>>> LaTeXIter<'a, T> {
    /// Wrap a Markdown `Event` iterator, turning it into a LaTeX stream.
    pub fn new(events: T) -> Self {
        LaTeXIter { events }
    }

    /// Convenience method for writing all output to an `fmt::Write`.
    pub fn write_to_fmt<W: fmt::Write>(&mut self, dest: &mut W) -> fmt::Result {
        for string in self {
            dest.write_str(&string)?
        }

        Ok(())
    }

    /// Convenience method for writing all output to an `io::Write`.
    pub fn write_to_io<W: io::Write>(&mut self, dest: &mut W) -> io::Result<()> {
        for string in self {
            dest.write_all(string.as_bytes())?
        }

        Ok(())
    }
}

impl<'a> LaTeXIter<'a, Parser<'a>> {
    /// Create a LaTeX event iterator from a Markdown string,
    /// with all `Parser` options enabled.
    pub fn with_str(string: &'a str) -> Self {
        Self::new(Parser::new_ext(string, Options::all()))
    }
}

impl<'a, T: Iterator<Item=Event<'a>>> Iterator for LaTeXIter<'a, T> {
    type Item = Cow<'a, str>;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
