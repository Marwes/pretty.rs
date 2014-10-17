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

fn nlspace<S>(outs:|S,String| -> S, s:S, i:uint) -> S {
    outs(s, pad_right(' ', i + String::from_str("\n").len(), String::from_str("\n")))
}

fn spaces<S>(outs:|S,String| -> S, s:S, i:uint) -> S {
    outs(s, pad_left(' ', i, String::from_str("")))
}

fn fitting(xs:&Vec<(uint,mode::Mode,Doc)>, left:uint) -> bool {
    if left as int >= 0 {
        match xs.as_slice() {
            [] => true,
            [(ref i, ref mode, ref doc), ref rest..] => match doc {
                &Nil => fitting(&rest.to_vec(), left),
                &Append(ref x, ref y) => {
                    let mut ys = [(*i,*mode,*x.clone()), (*i,*mode,*y.clone())].to_vec();
                    ys.push_all(*rest);
                    fitting(&ys,left)
                },
                &Nest(j, ref x) => {
                    let mut ys = [(*i+j,*mode,*x.clone())].to_vec();
                    ys.push_all(*rest);
                    fitting(&ys, left)
                },
                &Text(ref str) => fitting(&rest.to_vec(), left - str.len()),
                &Break(sp, _) => match *mode {
                    mode::Flat => fitting(&rest.to_vec(), left - sp),
                    mode::Break => true
                },
                &Newline => true,
                &Group(ref x) => {
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

fn best<S:Clone>(w:uint, outs:|S, String| -> S, s:S, x:Doc) -> S {
    fn go<S:Clone>(w:uint, outs:|S,String| -> S, s:S, k:uint, xs:Vec<(uint,mode::Mode,Doc)>) -> S {
        match xs.as_slice() {
            [] => s.clone(),
            [(ref i, ref mode, ref doc), ref rest..] => match doc {
                &Nil => go(w, outs, s, k, rest.to_vec()),
                &Append(ref x, ref y) => {
                    let mut rest2 = [(*i,*mode,*x.clone()), (*i,*mode,*y.clone())].to_vec();
                    rest2.push_all(*rest);
                    go(w, outs, s, k, rest2)
                },
                &Nest(j, ref x) => {
                    let mut rest2 = [(*i+j,*mode,*x.clone())].to_vec();
                    rest2.push_all(*rest);
                    go(w, outs, s, k, rest2)
                },
                &Text(ref str) => {
                    let ss = outs(s, str.clone());
                    go(w, outs, ss, (k + str.len()), rest.to_vec())
                },
                &Newline => {
                    let ss = nlspace(|x,y| outs(x,y), s, *i);
                    go(w, outs, ss, *i, rest.to_vec())
                }
                &Break(sp, off) => {
                    match *mode {
                        mode::Flat => {
                            let ss = spaces(|x,y| outs(x,y), s.clone(), sp);
                            go(w, outs, ss, k + sp, rest.to_vec())
                        },
                        mode::Break => {
                            let ss = nlspace(|x,y| outs(x,y), s.clone(), i + off);
                            go(w, outs, ss, i + off, rest.to_vec())
                        }
                    }
                },
                &Group(ref x) => {
                    match *mode {
                        mode::Flat => {
                            let mut rest2 = [(*i,mode::Flat,*x.clone())].to_vec();
                            rest2.push_all(*rest);
                            go(w, outs, s, k, rest2)
                        },
                        mode::Break => {
                            let mut rest2 = [(*i,mode::Flat,*x.clone())].to_vec();
                            rest2.push_all(*rest);
                            if fitting(&rest2, w - k) {
                                go(w, outs, s, k, rest2)
                            } else {
                                let mut rest3 = [(*i,mode::Break,*x.clone())].to_vec();
                                rest3.push_all(*rest);
                                go(w, outs, s, k, rest3)
                            }
                        }
                    }
                }
            }
        }
    }
    go(w, outs, s, 0, [(0,mode::Break,x)].to_vec())
}

impl Doc {
    pub fn to_string(&self, w:uint) -> String {
        let outs = |strs:Vec<String>, s:String| -> Vec<String> {
            let mut welp = [s].to_vec();
            welp.push_all(strs.as_slice()); // horrible
            welp
        };

        let mut strs = best(w, outs, [].to_vec(), self.clone());
        strs.reverse();
        strs.push(String::from_str("\n"));
        strs.concat()
    }
}

fn main() {
}
