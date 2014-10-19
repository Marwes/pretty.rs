fn pad_right(c:char, l:uint, mut str:String) -> String {
    let str_len = str.len();
    if l > str_len {
        let padding =
            Vec::from_elem(l - str_len, String::from_chars([c]))
                .concat();
        // let mut res = str.clone();
        str.push_str(padding.as_slice());
        str
    } else {
        str
    }
}

fn pad_left(c:char, l:uint, str:String) -> String {
    let str_len = str.len();
    if l > str_len {
        let mut res =
            Vec::from_elem(l - str_len, String::from_chars([c]))
                .concat();
        res.push_str(str.as_slice());
        res
    } else {
        str
    }
}

pub fn nl_space(i:uint) -> String {
    pad_right(' ', i + String::from_str("\n").len(), String::from_str("\n"))
}

pub fn spaces(i:uint) -> String {
    pad_left(' ', i, String::from_str(""))
}
