#![allow(dead_code)]
#![allow(unused_variable)]

#[deriving(Show)]
#[deriving(Clone)]
pub enum Doc {
    Nil,
    Append(Box<Doc>, Box<Doc>),
    Nest(uint, Box<Doc>),
    Text(String),
    Break(uint, uint),
    Newline,
    Group(Box<Doc>)
}

impl Doc {
    pub fn append(&self, e:Doc) -> Doc {
        match self {
            &Nil => e,
            x => match e {
                Nil => x.clone(),
                y => Append(box x.clone(), box y)
            }
        }
    }
}

fn concat(ds:&[Doc]) -> Doc {
    ds.iter().fold(Nil, |a, b| Append(box a, box b.clone()))
}

fn int(i:int) -> Doc {
    Text(format!("{}", i))
}

fn char(c:char) -> Doc {
    Text(format!("{}", c))
}

fn bool(b:bool) -> Doc {
    if b {
        Text(String::from_str("true"))
    } else {
        Text(String::from_str("false"))
    }
}

mod mode {
    #[deriving(Clone)]
    pub enum Mode {
        Flat,
        Break
    }
}

fn replicate<A:Clone>(x:A, l:uint) -> Vec<A> {
    let i = 0u;
    let mut res = Vec::new();
    for _ in range(i,l) {
        res.push(x.clone());
    }
    res
}
fn pad_right(c:char, l:uint, str:String) -> String {
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
fn pad_left(c:char, l:uint, str:String) -> String {
    let str_len = str.len();
    if l > str_len {
      let mut res = replicate(String::from_chars([c]), l - str_len).concat();
      res.push_str(str.as_slice());
      res
    } else {
        str
    }
}

fn nlspace(s:Vec<String>, i:uint) -> Vec<String> {
    prepend(s, pad_right(' ', i + String::from_str("\n").len(), String::from_str("\n")))
}

fn spaces(s:Vec<String>, i:uint) -> Vec<String> {
    prepend(s, pad_left(' ', i, String::from_str("")))
}

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
    let mut res = [x].to_vec();
    res.push_all(v.as_slice());
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
                Text(ref str) => {
                    let ss = prepend(s, str.clone());
                    go(w, ss, k + str.len(), rest.to_vec())
                },
                Newline => {
                    let ss = nlspace(s, *i);
                    go(w, ss, *i, rest.to_vec())
                },
                Break(sp, off) => {
                    match *mode {
                        mode::Flat => {
                            let ss = spaces(s.clone(), sp);
                            go(w, ss, k + sp, rest.to_vec())
                        },
                        mode::Break => {
                            let ss = nlspace(s.clone(), i + off);
                            go(w, ss, i + off, rest.to_vec())
                        }
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
    pub fn to_string(&self, w:uint) -> String {
        let mut strs = best(w, [].to_vec(), self.clone());
        strs.reverse();
        strs.push(String::from_str("\n"));
        strs.concat()
    }
}
