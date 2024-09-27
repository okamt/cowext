use std::{borrow::Cow, ops::Index, slice::SliceIndex};

/// Extension methoods for [`Cow<'_, str>`]-like types.
pub trait CowStrExt: Sized {
    /// Slices the string in place.
    ///
    /// # Panics
    ///
    /// May panic if the index is out of bounds.
    fn shrink<I>(&mut self, index: I)
    where
        I: SliceIndex<str, Output = str>;

    /// Removes the last character from the string buffer and returns it.
    ///
    /// Returns None if the string is empty.
    fn pop(&mut self) -> Option<char>;

    /// Divides the string into two at the given byte index.
    ///
    /// # Panics
    ///
    /// Panics if `at` is not on a UTF-8 code point boundary, or if it is
    /// past the end of the last code point of the string.
    fn split_at(self, at: usize) -> (Self, Self);
}

impl<'a> CowStrExt for Cow<'a, str> {
    fn shrink<I>(&mut self, index: I)
    where
        I: SliceIndex<str, Output = str>,
    {
        match self {
            Cow::Borrowed(s) => *s = s.index(index),
            Cow::Owned(s) => *s = s.index(index).to_string(),
        }
    }

    fn pop(&mut self) -> Option<char> {
        match self {
            Cow::Borrowed(s) => {
                let mut chars = s.chars();
                let result = chars.next_back()?;
                *s = chars.as_str();
                Some(result)
            }
            Cow::Owned(s) => s.pop(),
        }
    }

    fn split_at(self, at: usize) -> (Self, Self) {
        match self {
            Cow::Borrowed(s) => {
                let (left, right) = s.split_at(at);
                (Cow::Borrowed(left), Cow::Borrowed(right))
            }
            Cow::Owned(mut s) => {
                let rest = s.split_off(at);
                (Cow::Owned(s), Cow::Owned(rest))
            }
        }
    }
}
