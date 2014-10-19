use super::mode;
use super::string_utils;
use std::collections::DList;
use std::collections::Deque;

#[deriving(Clone)]
#[deriving(Show)]
enum DOC {
    Nil,
    Append(Box<Doc>, Box<Doc>),
    Break(uint, uint),
    Group(Box<Doc>),
    Nest(uint, Box<Doc>),
    Newline,
    Text(String),
}

pub type Doc = DOC;

fn fitting(xs:DList<(uint,mode::Mode,Doc)>, left:uint) -> bool {
    let mut tail = xs.clone();
    match tail.pop_front() {
        None => {
            true
        },
        Some((i, mode, ref doc)) => match doc.clone() {
            Nil => {
                fitting(tail, left)
            },
            Append(box x, box y) => {
                let mut prefix = DList::new();
                prefix.push((i, mode, x));
                prefix.push((i, mode, y));
                tail.prepend(prefix);
                fitting(tail, left)
            },
            Nest(j, box x) => {
                let mut prefix = DList::new();
                prefix.push((i + j, mode, x));
                tail.prepend(prefix);
                fitting(tail, left)
            },
            Text(str) => {
                fitting(tail, left - str.len())
            },
            Break(sp, _) => match mode {
                mode::Flat => {
                    fitting(tail, left - sp)
                },
                mode::Break => {
                    true
                },
            },
            Newline => {
                true
            },
            Group(box x) => {
                let mut prefix = DList::new();
                prefix.push((i, mode, x));
                tail.prepend(prefix);
                fitting(tail, left)
            },
        }
    }
}

fn best(w:uint, s:DList<String>, x:Doc) -> DList<String> {
    let mut result = s.clone();

    fn go(w:uint, s:&mut DList<String>, k:uint, xs:&mut DList<(uint,mode::Mode,Doc)>) {
        match xs.pop_front() {
            None => { },
            Some((i, mode, ref doc)) => match doc.clone() {
                Nil => {
                    go(w, s, k, xs)
                },
                Append(box x, box y) => {
                    let mut prefix = DList::new();
                    prefix.push((i, mode, x));
                    prefix.push((i, mode, y));
                    xs.prepend(prefix);
                    go(w, s, k, xs)
                },
                Nest(j, box x) => {
                    let mut prefix = DList::new();
                    prefix.push((i + j, mode, x));
                    xs.prepend(prefix);
                    go(w, s, k, xs)
                },
                Text(ref str) => {
                    let mut prefix = DList::new();
                    prefix.push(str.clone());
                    s.prepend(prefix);
                    go(w, s, k + str.len(), xs)
                },
                Newline => {
                    let mut prefix = DList::new();
                    prefix.push(string_utils::nl_space(i));
                    s.prepend(prefix);
                    go(w, s, i, xs)
                },
                Break(sp, off) => match mode {
                    mode::Flat => {
                        let mut prefix = DList::new();
                        prefix.push(string_utils::spaces(sp));
                        s.prepend(prefix);
                        go(w, s, k + sp, xs)
                    },
                    mode::Break => {
                        let mut prefix = DList::new();
                        prefix.push(string_utils::nl_space(i + off));
                        s.prepend(prefix);
                        go(w, s, i + off, xs)
                    }
                },
                Group(ref x) => match mode {
                    mode::Flat => {
                        let mut prefix = DList::new();
                        prefix.push((i, mode::Flat, *x.clone()));
                        xs.prepend(prefix);
                        go(w, s, k, xs)
                    },
                    mode::Break => {
                        let mut ys = xs.clone();
                        let mut flat_prefix = DList::new();
                        flat_prefix.push((i, mode::Flat, *x.clone()));
                        ys.prepend(flat_prefix);
                        if fitting(ys, w - k) {
                            let mut prefix = DList::new();
                            prefix.push((i, mode::Flat, *x.clone()));
                            xs.prepend(prefix);
                            go(w, s, k, xs)
                        } else {
                            let mut prefix = DList::new();
                            prefix.push((i, mode::Break, *x.clone()));
                            xs.prepend(prefix);
                            go(w, s, k, xs)
                        }
                    }
                }
            }
        }
    }

    let mut start = DList::new();
    start.push((0, mode::Break, x));

    go(w, &mut result, 0, &mut start);
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

    pub fn brk(space:uint, offset:uint) -> Doc {
        Break(space, offset)
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

    pub fn as_str<T:ToString>(t:T) -> Doc {
        Doc::text(t.to_string())
    }

    pub fn to_string(self, w:uint) -> String {
        let strs = best(w, DList::new(), self);

        let mut result = String::new();
        for str in strs.iter().rev() {
            result.push_str(str.as_slice());
        }
        result
    }
}
