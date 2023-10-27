use subslice::SubsliceExt;
use widestring::Utf32Str;

use crate::TokenRange;

pub(crate) fn search(haystack: &Utf32Str, needle: &Utf32Str) -> Option<TokenRange> {
    search_slice(haystack.as_slice(), needle.as_slice())
}

pub(crate) fn search_slice(haystack: &[u32], needle: &[u32]) -> Option<TokenRange> {
    haystack
        .find(needle)
        .map(|offset| TokenRange::new(offset, needle.len()))
}
