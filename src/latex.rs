//! Generating LaTeX output.

use std::fmt;
use std::io;
use std::borrow::Cow;
use pulldown_cmark::{ Parser, Options, Event };
use error::Result;

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
    pub fn write_to_fmt<W: fmt::Write>(&mut self, dest: &mut W) -> Result<()> {
        for string in self {
            dest.write_str(&string?)?
        }

        Ok(())
    }

    /// Convenience method for writing all output to an `io::Write`.
    pub fn write_to_io<W: io::Write>(&mut self, dest: &mut W) -> Result<()> {
        for string in self {
            dest.write_all(string?.as_bytes())?
        }

        Ok(())
    }

    /// Convert a Markdown event to a LaTeX fragment.
    fn md_to_latex(event: Event) -> Result<Cow<str>> {
        use pulldown_cmark::Event::*;

        match event {
            Start(tag) => unimplemented!(),
            End(tag) => unimplemented!(),
            Text(text) => unimplemented!(),
            Html(_) => unimplemented!(),
            InlineHtml(_) => unimplemented!(),
            FootnoteReference(note) => unimplemented!(),
            SoftBreak => unimplemented!(),
            HardBreak => unimplemented!(),
        }
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
    type Item = Result<Cow<'a, str>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.events.next().map(Self::md_to_latex)
    }
}
