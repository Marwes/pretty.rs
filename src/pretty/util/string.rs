pub fn spaces(i:uint) -> String {
    String::from_char(i, ' ')
}

pub fn nl_spaces(i:uint) -> String {
    let mut str = String::from_str("\n");
    str.push_str(spaces(i).as_slice());
    str
}
