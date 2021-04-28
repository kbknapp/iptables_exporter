pub(crate) fn idx_after(start: usize, haystack: &str, needle: char) -> Option<usize> {
    haystack[start..].find(needle).map(|i| i + start)
}
