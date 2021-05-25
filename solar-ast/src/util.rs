pub unsafe fn from_to<'a>(start: &'a str, end: &'a str) -> &'a str {
    // TODO implement safety measures. Panic
    let length = end.as_ptr() as usize - start.as_ptr() as usize;

    let bytes = std::slice::from_raw_parts(start.as_ptr(), length);

    std::str::from_utf8_unchecked(bytes)
}
