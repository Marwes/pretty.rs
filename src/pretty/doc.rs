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
            Some((ind, mode, ref doc)) => match doc.clone() {
                Nil => {},
                Append(box lhs, box rhs) => {
                    let mut prefix = DList::new();
                    prefix.push((ind, mode, lhs));
                    prefix.push((ind, mode, rhs));
                    cmds.prepend(prefix);
                },
                Group(box doc) => {
                    let mut prefix = DList::new();
                    prefix.push((ind, mode, doc));
                    cmds.prepend(prefix);
                },
                Nest(off, box doc) => {
                    let mut prefix = DList::new();
                    prefix.push((ind + off, mode, doc));
                    cmds.prepend(prefix);
                },
                Newline => {
                    fits = true;
                },
                Text(str) => {
                    rem -= str.len() as int;
                },
            }
        }
    }

    fits
}

impl Doc {

    fn best(self:Doc, width:uint, mut buf:DList<String>) -> DList<String> {
        let mut pos: uint = 0;
        let mut cmds: DList<(uint,mode::Mode,Doc)> = DList::new();

        cmds.push((0, mode::Break, self));

        loop {
            match cmds.pop_front() {
                None => {
                    break;
                },
                Some((ind, mode, ref doc)) => match doc.clone() {
                    Nil => {},
                    Append(box lhs, box rhs) => {
                        let mut prefix = DList::new();
                        prefix.push((ind, mode, lhs));
                        prefix.push((ind, mode, rhs));
                        cmds.prepend(prefix);
                    },
                    Group(box doc) => match mode {
                        mode::Flat => {
                            let mut prefix = DList::new();
                            prefix.push((ind, mode::Flat, doc));
                            cmds.prepend(prefix);
                        },
                        mode::Break => {
                            let mut cmds_dup = cmds.clone();
                            let mut flat_prefix = DList::new();
                            flat_prefix.push((ind, mode::Flat, doc.clone()));
                            cmds_dup.prepend(flat_prefix);
                            if fitting(cmds_dup, width as int - pos as int) {
                                let mut prefix = DList::new();
                                prefix.push((ind, mode::Flat, doc));
                                cmds.prepend(prefix);
                            } else {
                                let mut prefix = DList::new();
                                prefix.push((ind, mode::Break, doc));
                                cmds.prepend(prefix);
                            }
                        }
                    },
                    Nest(off, box doc) => {
                        let mut prefix = DList::new();
                        prefix.push((ind + off, mode, doc));
                        cmds.prepend(prefix);
                    },
                    Newline => {
                        let mut prefix = DList::new();
                        prefix.push(util::string::nl_spaces(ind));
                        buf.prepend(prefix);
                        pos = ind;
                    },
                    Text(str) => {
                        let mut prefix = DList::new();
                        prefix.push(str.clone());
                        buf.prepend(prefix);
                        pos += str.len();
                    },
                }
            }
        }

        buf
    }

    #[inline]
    pub fn nil() -> Doc {
        Nil
    }

    #[inline]
    pub fn append(self, that:Doc) -> Doc {
        match self {
            Nil => that,
            lhs => match that {
                Nil => lhs,
                rhs => Append(box lhs, box rhs)
            }
        }
    }

    #[inline]
    pub fn as_string<T:ToString>(t:T) -> Doc {
        Doc::text(t.to_string())
    }

    #[inline]
    pub fn concat(ds:&[Doc]) -> Doc {
        ds.iter().fold(Nil, |a, b| a.append(b.clone()))
    }

    #[inline]
    pub fn group(self) -> Doc {
        Group(box self)
    }

    #[inline]
    pub fn nest(self, off:uint) -> Doc {
        Nest(off, box self)
    }

    #[inline]
    pub fn newline() -> Doc {
        Newline
    }

    #[inline]
    pub fn render(self, width:uint) -> String {
        let mut result = String::new();
        for str in self.best(width, DList::new()).iter().rev() {
            result.push_str(str.as_slice());
        }
        result
    }

    #[inline]
    pub fn text<S:Str>(str:S) -> Doc {
        Text(String::from_str(str.as_slice()))
    }

}
