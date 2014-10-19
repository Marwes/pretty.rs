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

fn fitting(mut cmds: DList<(uint,mode::Mode,Doc)>, mut rem:int) -> bool {
    let mut fits = true;

    loop {
        if rem < 0 {
            fits = false;
            break;
        }
        match cmds.pop_front() {
            None => {
                break;
            },
            Some((i, mode, ref doc)) => match doc.clone() {
                Nil => {},
                Append(box x, box y) => {
                    let mut prefix = DList::new();
                    prefix.push((i, mode, x));
                    prefix.push((i, mode, y));
                    cmds.prepend(prefix);
                },
                Nest(j, box x) => {
                    let mut prefix = DList::new();
                    prefix.push((i + j, mode, x));
                    cmds.prepend(prefix);
                },
                Text(str) => {
                    rem -= str.len() as int;
                },
                Newline => {
                    fits = true;
                },
                Group(box x) => {
                    let mut prefix = DList::new();
                    prefix.push((i, mode, x));
                    cmds.prepend(prefix);
                },
            }
        }
    }

    fits
}

fn best(width:uint, mut buf:DList<String>, x:Doc) -> DList<String> {
    let mut pos: uint = 0;
    let mut cmds: DList<(uint,mode::Mode,Doc)> = DList::new();

    cmds.push((0, mode::Break, x));

    loop {
        match cmds.pop_front() {
            None => {
                break;
            },
            Some((i, mode, ref doc)) => match doc.clone() {
                Nil => {},
                Append(box x, box y) => {
                    let mut prefix = DList::new();
                    prefix.push((i, mode, x));
                    prefix.push((i, mode, y));
                    cmds.prepend(prefix);
                },
                Nest(j, box x) => {
                    let mut prefix = DList::new();
                    prefix.push((i + j, mode, x));
                    cmds.prepend(prefix);
                },
                Text(str) => {
                    let mut prefix = DList::new();
                    prefix.push(str.clone());
                    buf.prepend(prefix);
                    pos += str.len();
                },
                Newline => {
                    let mut prefix = DList::new();
                    prefix.push(util::string::nl_spaces(i));
                    buf.prepend(prefix);
                    pos = i;
                },
                Group(ref x) => match mode {
                    mode::Flat => {
                        let mut prefix = DList::new();
                        prefix.push((i, mode::Flat, *x.clone()));
                        cmds.prepend(prefix);
                    },
                    mode::Break => {
                        let mut cmds_dup = cmds.clone();
                        let mut flat_prefix = DList::new();
                        flat_prefix.push((i, mode::Flat, *x.clone()));
                        cmds_dup.prepend(flat_prefix);
                        if fitting(cmds_dup, width as int - pos as int) {
                            let mut prefix = DList::new();
                            prefix.push((i, mode::Flat, *x.clone()));
                            cmds.prepend(prefix);
                        } else {
                            let mut prefix = DList::new();
                            prefix.push((i, mode::Break, *x.clone()));
                            cmds.prepend(prefix);
                        }
                    }
                }
            }
        }
    }

    buf
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
