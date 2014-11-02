#[inline]
pub fn spaces(n:uint) -> String {
    String::from_char(n, ' ')
}

#[inline]
pub fn nl_spaces(n:uint) -> String {
    let mut str = String::from_str("\n");
    str.push_str(spaces(n).as_slice());
    str
}
