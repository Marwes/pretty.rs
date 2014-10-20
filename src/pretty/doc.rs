use super::mode;
use super::util;
use std::collections::Deque;
use std::collections::RingBuf;

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
type Cmd<'a> = (uint,mode::Mode,&'a Doc);

fn fitting(mut cmds:RingBuf<Cmd>, mut rem:int) -> bool {
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
            Some((ind, mode, doc)) => match doc {
                &Nil => {
                },
                &Append(box ref ldoc, box ref rdoc) => {
                    cmds.push_front((ind, mode, rdoc));
                    cmds.push_front((ind, mode, ldoc));
                },
                &Group(box ref doc) => {
                    cmds.push_front((ind, mode, doc));
                },
                &Nest(off, box ref doc) => {
                    cmds.push_front((ind + off, mode, doc));
                },
                &Newline => {
                    fits = true;
                },
                &Text(ref str) => {
                    rem -= str.len() as int;
                },
            }
        }
    }

    fits
}

impl Doc {

    fn best(&self, width:uint) -> Vec<String> {
        let mut pos = 0u;
        let mut cmds = RingBuf::new();
        let mut result = Vec::new();

        cmds.push((0, mode::Break, self));

        loop {
            match cmds.pop_front() {
                None => {
                    break;
                },
                Some((ind, mode, doc)) => match doc {
                    &Nil => {
                    },
                    &Append(box ref ldoc, box ref rdoc) => {
                        cmds.push_front((ind, mode, rdoc));
                        cmds.push_front((ind, mode, ldoc));
                    },
                    &Group(box ref doc) => match mode {
                        mode::Flat => {
                            cmds.push_front((ind, mode::Flat, doc));
                        },
                        mode::Break => {
                            // FIXME: fitting() modifies cmds so we must either
                            // take a clone and pass it or rewrite fitting()
                            // to modify a separate cmd stack and only observe
                            // this one.
                            let mut cmds_dup = cmds.clone();
                            let next = (ind, mode::Flat, doc);
                            cmds_dup.push_front(next);
                            if fitting(cmds_dup, width as int - pos as int) {
                                cmds.push_front(next);
                            } else {
                                cmds.push_front((ind, mode::Break, doc));
                            }
                        }
                    },
                    &Nest(off, box ref doc) => {
                        cmds.push_front((ind + off, mode, doc));
                    },
                    &Newline => {
                        result.push(util::string::nl_spaces(ind));
                        pos = ind;
                    },
                    &Text(ref str) => {
                        // FIXME: we have to clone here otherwise result would
                        // have to contain String and &String. We cannot make
                        // Newline case return &String because the region is
                        // limited to the scope of its arm; it does not live
                        // long enough (the strings in Text will outlast).
                        result.push(str.clone());
                        pos += str.len();
                    },
                }
            }
        }

        result
    }

    #[inline]
    pub fn nil() -> Doc {
        Nil
    }

    #[inline]
    pub fn append(self, that:Doc) -> Doc {
        match self {
            Nil => {
                that
            },
            ldoc => match that {
                Nil => {
                    ldoc
                },
                rdoc => {
                    Append(box ldoc, box rdoc)
                },
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

    // FIXME: could possibly parallelize this part since string append should
    // be associative. Might be a good example for rust-monoid…
    #[inline]
    pub fn render(&self, width:uint) -> String {
        let mut result = String::new();
        for str in self.best(width).iter() {
            result.push_str(str.as_slice());
        }
        result
    }

    #[inline]
    pub fn text<T:Str>(str:T) -> Doc {
        Text(String::from_str(str.as_slice()))
    }

}
