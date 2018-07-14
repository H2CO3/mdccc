//! Escaping text for use inside LaTeX.

use std::borrow::Cow;

/// LaTeX-escapes arbitrary text. TODO(H2CO3): are there more
/// characters we need to care about?
pub fn latex(text: Cow<str>) -> Cow<str> {
    if text.contains(['#', '%', '_'].as_ref()) {
        text
            .replace('#', r"\#")
            .replace('%', r"\%")
            .replace('_', r"\_")
            .into()
    } else {
        text
    }
}
