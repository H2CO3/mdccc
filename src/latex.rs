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
    /// Whether the document prologue should be emitted.
    emit_prologue: bool,
    /// Whether the document epilogue should be emitted.
    emit_epilogue: bool,
}

impl<'a, T: Iterator<Item=Event<'a>>> LaTeXIter<'a, T> {
    /// Wrap a Markdown `Event` iterator, turning it into a LaTeX stream.
    /// If `wrap_document` is true, a document prologue and epilogue will
    /// be emitted before and after the generated LaTeX code.
    pub fn new(events: T, wrap_document: bool) -> Self {
        LaTeXIter {
            events: events,
            emit_prologue: wrap_document,
            emit_epilogue: wrap_document,
        }
    }

    /// Convenience method for writing all output to an `fmt::Write`.
    pub fn write_to_fmt<W: fmt::Write>(self, dest: &mut W) -> Result<()> {
        for string in self {
            dest.write_str(&string?)?
        }

        Ok(())
    }

    /// Convenience method for writing all output to an `io::Write`.
    pub fn write_to_io<W: io::Write>(self, dest: &mut W) -> Result<()> {
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
            Tag::Paragraph => Ok(Cow::from("\n\n")),
            Tag::Rule => Ok(Cow::from("\n\n\\hrulefill\n\n")),
            Tag::Header(level) => Ok(Cow::from(match level {
                1 => r"\textbf{\Huge ",
                2 => r"\textbf{\huge ",
                3 => r"\textbf{\LARGE ",
                4 => r"\textbf{\Large ",
                5 => r"\textbf{\large ",
                _ => r"\textbf{\normalsize ",
            })),
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
            Tag::Paragraph => Ok(Cow::default()),
            Tag::Rule => Ok(Default::default()),
            Tag::Header(_level) => Ok(Cow::from("}\n\n")),
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
    /// Create a LaTeX event iterator from a Markdown string, with all `Parser`
    /// options enabled, emitting a document prologue and epilogue.
    pub fn with_str(string: &'a str) -> Self {
        Self::new(Parser::new_ext(string, Options::all()), true)
    }
}

impl<'a, T: Iterator<Item=Event<'a>>> Iterator for LaTeXIter<'a, T> {
    type Item = Result<Cow<'a, str>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.emit_prologue {
            self.emit_prologue = false;
            return Some(Ok(Cow::from(DOCUMENT_PROLOGUE)));
        }

        if let Some(event) = self.events.next() {
            return Some(Self::md_to_latex(event));
        }

        if self.emit_epilogue {
            self.emit_epilogue = false;
            return Some(Ok(Cow::from(DOCUMENT_EPILOGUE)));
        }

        None
    }
}

/// The optional document prologue.
static DOCUMENT_PROLOGUE: &str = r"
\documentclass[fontisze=11pt]{scrreprt}

\usepackage{lmodern}
\usepackage[utf8]{inputenc}
\usepackage[T1]{fontenc}
\usepackage[onehalfspacing]{setspace}
\usepackage[margin=1.5cm]{geometry}
\usepackage[style=english]{csquotes}
\usepackage{listings}
\usepackage[format=plain,font=footnotesize]{caption}

\lstset{
  columns=fixed,
  basicstyle=\ttfamily\color{black},
  basewidth=0.5em,
  breaklines=true,
  frame=leftline,
  numbers=left,
  numbersep=5pt,
  numberstyle=\color{mygreen},
  keywordstyle=\color{blue},
  commentstyle=\color{mygray},
  showspaces=false,
  showstringspaces=false,
  stringstyle=\color{orange},
}

\begin{document}
";
/// The optional document epilogue.
static DOCUMENT_EPILOGUE: &str = r"\end{document}";
