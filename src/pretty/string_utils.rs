pub fn replicate<A:Clone>(x:A, l:uint) -> Vec<A> {
    let i = 0u;
    let mut res = Vec::new();
    for _ in range(i,l) {
        res.push(x.clone());
    }
    res
}

pub fn pad_right(c:char, l:uint, str:String) -> String {
    let str_len = str.len();
    if l > str_len {
        let padding = replicate(String::from_chars([c]), l - str_len).concat();
        let mut res = str.clone();
        res.push_str(padding.as_slice());
        res
    } else {
        str
    }
}

pub fn pad_left(c:char, l:uint, str:String) -> String {
    let str_len = str.len();
    if l > str_len {
      let mut res = replicate(String::from_chars([c]), l - str_len).concat();
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

