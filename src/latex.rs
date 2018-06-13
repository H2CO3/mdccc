//! Generating LaTeX output.

use std::fmt;
use std::io;
use std::borrow::Cow;
use pulldown_cmark::{ Parser, Options, Event, Tag };
use error::{ Error, Result };
use escape;

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
            Start(tag) => Self::start_tag(tag),
            End(tag) => Self::end_tag(tag),
            Text(text) => Ok(escape::latex(text)),
            Html(_) => Err(Error::new("don't know how to convert HTML to LaTeX")),
            InlineHtml(_) => Err(Error::new("don't know how to convert HTML to LaTeX")),
            FootnoteReference(_note) => Err(Error::new("don't know how to render footnotes yet")),
            SoftBreak => Ok(Cow::from("\\newline\n")),
            HardBreak => Ok(Cow::from("\n\n"))
        }
    }

    /// Convert a Markdown start event to a LaTeX fragment.
    fn start_tag(tag: Tag) -> Result<Cow<str>> {
        match tag {
            Tag::Paragraph => Ok(Default::default()),
            Tag::Rule => Ok(Default::default()),
            Tag::Header(_level) => Ok(Default::default()),
            Tag::BlockQuote => Ok(Default::default()),
            Tag::CodeBlock(_text) => Ok(Default::default()),
            Tag::List(_fstidx) => Ok(Default::default()),
            Tag::Item => Ok(Default::default()),
            Tag::FootnoteDefinition(_footnote) => Ok(Default::default()),
            Tag::Table(_alignments) => Ok(Default::default()),
            Tag::TableHead => Ok(Default::default()),
            Tag::TableRow => Ok(Default::default()),
            Tag::TableCell => Ok(Default::default()),
            Tag::Emphasis => Ok(Cow::from(r"\textit{")),
            Tag::Strong => Ok(Cow::from(r"\textbf{")),
            Tag::Code => Ok(Default::default()),
            Tag::Link(_url, _label) => Ok(Default::default()),
            Tag::Image(_url, _label) => Ok(Default::default()),
        }
    }

    /// Convert a Markdown end event to a LaTeX fragment.
    fn end_tag(tag: Tag) -> Result<Cow<str>> {
        match tag {
            Tag::Paragraph => Ok(Default::default()),
            Tag::Rule => Ok(Default::default()),
            Tag::Header(_level) => Ok(Default::default()),
            Tag::BlockQuote => Ok(Default::default()),
            Tag::CodeBlock(_text) => Ok(Default::default()),
            Tag::List(_fstidx) => Ok(Default::default()),
            Tag::Item => Ok(Default::default()),
            Tag::FootnoteDefinition(_footnote) => Ok(Default::default()),
            Tag::Table(_alignments) => Ok(Default::default()),
            Tag::TableHead => Ok(Default::default()),
            Tag::TableRow => Ok(Default::default()),
            Tag::TableCell => Ok(Default::default()),
            Tag::Emphasis => Ok(Cow::from("}")),
            Tag::Strong => Ok(Cow::from("}")),
            Tag::Code => Ok(Default::default()),
            Tag::Link(_url, _label) => Ok(Default::default()),
            Tag::Image(_url, _label) => Ok(Default::default()),
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
