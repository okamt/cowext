use std::borrow::Cow;

/// Extension methoods for [`Cow<'_, str>`]-like types.
pub trait CowStrExt: Sized {
    /// Divides [`Self`] into two at the given byte index.
    ///
    /// # Panics
    /// Panics if `at` is not on a UTF-8 code point boundary, or if it is
    /// past the end of the last code point of the string.
    fn split_at(self, at: usize) -> (Self, Self);
}

impl<'a> CowStrExt for Cow<'a, str> {
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
