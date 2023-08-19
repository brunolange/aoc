pub fn split_on_uppercase(s: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let mut start = 0;

    for (i, c) in s.char_indices() {
        if c.is_ascii_uppercase() {
            if start < i {
                result.push(&s[start..i]);
            }
            start = i;
        }
    }

    if start < s.len() {
        result.push(&s[start..]);
    }

    result
}
