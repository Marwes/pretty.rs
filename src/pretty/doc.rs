#![allow(dead_code)]
#![allow(unused_variable)]

use pretty::mode;
use pretty::string_utils;

#[deriving(Show)]
#[deriving(Clone)]
enum DOC {
    Nil,
    Append(Box<Doc>, Box<Doc>),
    Nest(uint, Box<Doc>),
    Text(String),
    Break(uint, uint),
    Newline,
    Group(Box<Doc>)
}

pub type Doc = DOC;

fn fitting(xs:&Vec<(uint,mode::Mode,Doc)>, left:uint) -> bool {
    if left as int >= 0 {
        match xs.as_slice() {
            [] => true,
            [(ref i, ref mode, ref doc), ref rest..] => match *doc {
                Nil => fitting(&rest.to_vec(), left),
                Append(ref x, ref y) => {
                    let mut ys = [(*i,*mode,*x.clone()), (*i,*mode,*y.clone())].to_vec();
                    ys.push_all(*rest);
                    fitting(&ys,left)
                },
                Nest(j, ref x) => {
                    let mut ys = [(*i+j,*mode,*x.clone())].to_vec();
                    ys.push_all(*rest);
                    fitting(&ys, left)
                },
                Text(ref str) => fitting(&rest.to_vec(), left - str.len()),
                Break(sp, _) => match *mode {
                    mode::Flat => fitting(&rest.to_vec(), left - sp),
                    mode::Break => true
                },
                Newline => true,
                Group(ref x) => {
                    let mut ys = [(*i,*mode, *x.clone())].to_vec();
                    ys.push_all(*rest);
                    fitting(&ys, left)
                }
            }
        }
    }
    else {
        false
    }
}

fn prepend<A:Clone>(v: Vec<A>, x:A) -> Vec<A> {
    let mut res = v;
    res.insert(0,x);
    res
}

fn best(w:uint, s:Vec<String>, x:Doc) -> Vec<String> {
    fn go(w:uint, s:Vec<String>, k:uint, xs:Vec<(uint,mode::Mode,Doc)>) -> Vec<String> {
        match xs.as_slice() {
            [] => s.clone(),
            [(ref i, ref mode, ref doc), ref rest..] => match *doc {
                Nil => go(w, s, k, rest.to_vec()),
                Append(ref x, ref y) => {
                    let mut zs = [(*i, *mode, *x.clone()), (*i, *mode, *y.clone())].to_vec();
                    zs.push_all(*rest);
                    go(w, s, k, zs)
                },
                Nest(j, ref x) => {
                    let mut zs = [(*i + j, *mode, *x.clone())].to_vec();
                    zs.push_all(*rest);
                    go(w, s, k, zs)
                },
                Text(ref str) => go(w, prepend(s, str.clone()), k + str.len(), rest.to_vec()),
                Newline => go(w, prepend(s, string_utils::nl_space(*i)), *i, rest.to_vec()),
                Break(sp, off) => {
                    match *mode {
                        mode::Flat => go(w, prepend(s.clone(), string_utils::spaces(sp)), k + sp, rest.to_vec()),
                        mode::Break => go(w, prepend(s.clone(), string_utils::nl_space(i + off)), i + off, rest.to_vec())
                    }
                },
                Group(ref x) => {
                    match *mode {
                        mode::Flat => {
                            let mut zs = [(*i,mode::Flat,*x.clone())].to_vec();
                            zs.push_all(*rest);
                            go(w, s, k, zs)
                        },
                        mode::Break => {
                            let mut ys = [(*i,mode::Flat,*x.clone())].to_vec();
                            ys.push_all(*rest);
                            if fitting(&ys, w - k) {
                                go(w, s, k, ys)
                            } else {
                                let mut zs = [(*i,mode::Break,*x.clone())].to_vec();
                                zs.push_all(*rest);
                                go(w, s, k, zs)
                            }
                        }
                    }
                }
            }
        }
    }
    go(w, s, 0, [(0,mode::Break,x)].to_vec())
}

impl Doc {

    pub fn nil() -> Doc {
        Nil
    }

    pub fn append(&self, e:Doc) -> Doc {
        match *self {
            Nil => e,
            ref x => match e {
                Nil => x.clone(),
                y => Append(box x.clone(), box y)
            }
        }
    }

    pub fn nest(&self, i:uint) -> Doc {
        Nest(i, box self.clone())
    }

    pub fn text(str:String) -> Doc {
        Text(str)
    }

    pub fn brk(space:uint, offset:uint) -> Doc {
        Break(space, offset)
    }

    pub fn newline() -> Doc {
        Newline
    }

    pub fn group(&self) -> Doc {
        Group(box self.clone())
    }

    pub fn concat(ds:&[Doc]) -> Doc {
        ds.iter().fold(Nil, |a, b| a.append(b.clone()))
    }

    pub fn int(i:int) -> Doc {
        Text(format!("{}", i))
    }

    pub fn char(c:char) -> Doc {
        Text(format!("{}", c))
    }

    pub fn bool(b:bool) -> Doc {
        let res = if b { "true" } else { "false" };
        Text(String::from_str(res))
    }

    pub fn to_string(&self, w:uint) -> String {
        let mut strs = best(w, Vec::new(), self.clone());
        strs.reverse();
        strs.push(String::from_str("\n"));
        strs.concat()
    }
}

