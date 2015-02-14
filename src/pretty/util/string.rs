#[inline]
pub fn spaces(n:usize) -> String {
    use std::iter;
    iter::repeat(' ').take(n).collect()
}

#[inline]
pub fn nl_spaces(n:usize) -> String {
    let mut s = "\n".to_string();
    s.push_str(&spaces(n));
    s
}
