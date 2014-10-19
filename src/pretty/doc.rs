use super::mode;
use super::util;

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

fn fitting(xs:Vec<(uint,mode::Mode,Doc)>, left:uint) -> bool {
    match xs.as_slice() {
        [] => {
            true
        },
        [(i, mode, ref doc), rest..] => match doc.clone() {
            Nil => {
                fitting(rest.to_vec(), left)
            },
            Append(box x, box y) => {
                let mut ys = vec![(i, mode, x), (i, mode, y)];
                ys.push_all(rest);
                fitting(ys, left)
            },
            Nest(j, box x) => {
                let mut ys = vec![(i + j, mode, x)];
                ys.push_all(rest);
                fitting(ys, left)
            },
            Text(str) => {
                fitting(rest.to_vec(), left - str.len())
            },
            Break(sp, _) => match mode {
                mode::Flat => {
                    fitting(rest.to_vec(), left - sp)
                },
                mode::Break => {
                    true
                },
            },
            Newline => {
                true
            },
            Group(box x) => {
                let mut ys = vec![(i, mode, x)];
                ys.push_all(rest);
                fitting(ys, left)
            },
        }
    }
}

fn best(w:uint, s:Vec<String>, x:Doc) -> Vec<String> {
    fn go(w:uint, s:Vec<String>, k:uint, xs:Vec<(uint,mode::Mode,Doc)>) -> Vec<String> {
        match xs.as_slice() {
            [] => s,
            [(i, mode, ref doc), rest..] => match doc.clone() {
                Nil => {
                    go(w, s, k, rest.to_vec())
                },
                Append(box x, box y) => {
                    let mut zs = vec![(i, mode, x), (i, mode, y)];
                    zs.push_all(rest);
                    go(w, s, k, zs)
                },
                Nest(j, box x) => {
                    let mut zs = vec![(i + j, mode, x)];
                    zs.push_all(rest);
                    go(w, s, k, zs)
                },
                Text(str) => {
                    let s = util::vec::prepend(s, str.clone());
                    go(w, s, k + str.len(), rest.to_vec())
                },
                Newline => {
                    let wspace = util::string::nl_spaces(i);
                    let s = util::vec::prepend(s, wspace);
                    go(w, s, i, rest.to_vec())
                },
                Break(sp, off) => {
                    match mode {
                        mode::Flat => {
                            let wspace = util::string::spaces(sp);
                            let s = util::vec::prepend(s, wspace);
                            go(w, s, k + sp, rest.to_vec())
                        },
                        mode::Break => {
                            let wspace = util::string::nl_spaces(i + off);
                            let s = util::vec::prepend(s, wspace);
                            go(w, s, i + off, rest.to_vec())
                        }
                    }
                },
                Group(box x) => {
                    match mode {
                        mode::Flat => {
                            let mut zs = vec![(i, mode::Flat, x)];
                            zs.push_all(rest);
                            go(w, s, k, zs)
                        },
                        mode::Break => {
                            let mut ys = vec![(i, mode::Flat, x.clone())];
                            ys.push_all(rest);
                            if fitting(ys.clone(), w - k) {
                                go(w, s, k, ys)
                            } else {
                                let mut zs = vec![(i, mode::Break, x)];
                                zs.push_all(rest);
                                go(w, s, k, zs)
                            }
                        }
                    }
                }
            }
        }
    }
    go(w, s, 0, vec![(0, mode::Break, x)])
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

    // FIXME: perhaps call it line_break?
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

    pub fn as_string<T:ToString>(t:T) -> Doc {
        Doc::text(t.to_string())
    }

    pub fn render(self, w:uint) -> String {
        let mut strs = best(w, Vec::new(), self);
        strs.reverse();
        strs.push(String::from_str("\n"));
        strs.concat()
    }
}
