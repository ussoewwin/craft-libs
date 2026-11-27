//! The ink a coloured text range names, and the most text a part colours. Layer 2, so `primitives`
//! (layer 3) re-exports them rather than this adapter importing upward.

/// The longest text, in bytes, a part colours: a longer one is laid out in its plain ink, and its
/// [`ColorRanges`](crate::primitives::ColorRanges) is not called. It bounds the cost of an app's colouring (a
/// highlighter whose time grows with the square of the length on crafted input) at the size of a long script.
pub const COLORED_TEXT_LIMIT: usize = 16 * 1024;

/// A theme colour for a stretch of a field's text. Each is the token's colour moved, where it has to be, to the text
/// contrast minimum on the field's fill, so every ink reads in every theme and contrast.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum RangeInk {
    /// `text.muted`.
    Muted,
    /// `accent.text`.
    Accent,
    /// `status.good.fg`.
    Good,
    /// `status.warn.fg`.
    Warn,
    /// `status.bad.fg`.
    Bad,
    /// `status.info.fg`.
    Info,
}

/// The colour ranges of a text ([`TextOptions::color_ranges`](crate::primitives::TextOptions::color_ranges)): called
/// with the text each time the part lays it out, which is every frame the part shows. An app whose ranges are costly
/// to find keeps its own cache.
pub type ColorRanges = dyn Fn(&str) -> Vec<ColorRange>;

/// One stretch of a text in an ink of its own: `range` is in bytes of the text the [`ColorRanges`] was given.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ColorRange {
    /// The bytes the ink covers.
    pub range: std::ops::Range<usize>,
    /// Their ink.
    pub ink: RangeInk,
}

impl ColorRange {
    /// The bytes `range` in `ink`.
    pub fn new(range: std::ops::Range<usize>, ink: RangeInk) -> Self {
        ColorRange { range, ink }
    }
}
