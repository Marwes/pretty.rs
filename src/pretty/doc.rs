use super::mode;
use super::util;
use std::collections::DList;
use std::collections::Deque;

#[deriving(Clone)]
#[deriving(Show)]
enum DOC {
    Nil,
    Append(Box<Doc>, Box<Doc>),
    Group(Box<Doc>),
    Nest(uint, Box<Doc>),
    Newline,
    Text(String),
}

pub type Doc = DOC;

fn fitting(xs:&mut DList<(uint,mode::Mode,Doc)>, initial_left:int) -> bool {
    let mut fits = true;

    let mut left = initial_left;

    loop {
        if left < 0 {
            fits = false;
            break;
        }

        match xs.pop_front() {
            None => {
                break;
            },
            Some((i, mode, ref doc)) => match doc.clone() {
                Nil => {},
                Append(box x, box y) => {
                    let mut prefix = DList::new();
                    prefix.push((i, mode, x));
                    prefix.push((i, mode, y));
                    xs.prepend(prefix);
                },
                Nest(j, box x) => {
                    let mut prefix = DList::new();
                    prefix.push((i + j, mode, x));
                    xs.prepend(prefix);
                },
                Text(str) => {
                    left -= str.len() as int;
                },
                Newline => {
                    fits = true;
                },
                Group(box x) => {
                    let mut prefix = DList::new();
                    prefix.push((i, mode, x));
                    xs.prepend(prefix);
                },
            }
        }
    }

    fits
}


fn best(w:uint, s:DList<String>, x:Doc) -> DList<String> {
    let mut start = DList::new();
    start.push((0u, mode::Break, x.clone()));

    let mut result = s.clone();
    let mut k = 0u;

    loop {
        match start.pop_front() {
            None => {
                break;
            },
            Some((i, mode, ref doc)) => match doc.clone() {
                Nil => {},
                Append(box x, box y) => {
                    let mut prefix = DList::new();
                    prefix.push((i, mode, x));
                    prefix.push((i, mode, y));
                    start.prepend(prefix);
                },
                Nest(j, box x) => {
                    let mut prefix = DList::new();
                    prefix.push((i + j, mode, x));
                    start.prepend(prefix);
                },
                Text(str) => {
                    let mut prefix = DList::new();
                    prefix.push(str.clone());
                    result.prepend(prefix);
                    k += str.len();
                },
                Newline => {
                    let mut prefix = DList::new();
                    prefix.push(util::string::nl_spaces(i));
                    result.prepend(prefix);
                    k = i;
                },
                Group(ref x) => match mode {
                    mode::Flat => {
                        let mut prefix = DList::new();
                        prefix.push((i, mode::Flat, *x.clone()));
                        start.prepend(prefix);
                    },
                    mode::Break => {
                        let mut ys = start.clone();
                        let mut flat_prefix = DList::new();
                        flat_prefix.push((i, mode::Flat, *x.clone()));
                        ys.prepend(flat_prefix);
                        if fitting(&mut ys, w as int - k as int) {
                            let mut prefix = DList::new();
                            prefix.push((i, mode::Flat, *x.clone()));
                            start.prepend(prefix);
                        } else {
                            let mut prefix = DList::new();
                            prefix.push((i, mode::Break, *x.clone()));
                            start.prepend(prefix);
                        }
                    }
                }
            }
        }
    }

    result
}

impl Doc {

    pub fn nil() -> Doc {
        Nil
    }

    pub fn append(self, e:Doc) -> Doc {
        match self {
            Nil => e,
            x => match e {
                Nil => x,
                y => Append(box x, box y)
            }
        }
    }

    pub fn nest(self, i:uint) -> Doc {
        Nest(i, box self)
    }

    pub fn text<S:Str>(s:S) -> Doc {
        Text(String::from_str(s.as_slice()))
    }

    pub fn newline() -> Doc {
        Newline
    }

    pub fn group(self) -> Doc {
        Group(box self)
    }

    pub fn concat(ds:&[Doc]) -> Doc {
        ds.iter().fold(Nil, |a, b| a.append(b.clone()))
    }

    pub fn as_string<T:ToString>(t:T) -> Doc {
        Doc::text(t.to_string())
    }

    pub fn render(self, w:uint) -> String {
        let strs = best(w, DList::new(), self);

        let mut result = String::new();
        for str in strs.iter().rev() {
            result.push_str(str.as_slice());
        }
        result
    }
}
