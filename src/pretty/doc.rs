use super::mode;
use super::util;

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

#[inline(always)]
fn fitting(mut cmds:Vec<Cmd>, mut rem:int) -> bool {
    let mut fits = true;

    loop {
        if rem < 0 {
            fits = false;
            break;
        }
        match cmds.pop() {
            None => {
                break;
            },
            Some((ind, mode, doc)) => match doc {
                &Nil => {
                },
                &Append(box ref ldoc, box ref rdoc) => {
                    cmds.push((ind, mode, rdoc));
                    cmds.push((ind, mode, ldoc));
                },
                &Group(box ref doc) => {
                    cmds.push((ind, mode, doc));
                },
                &Nest(off, box ref doc) => {
                    cmds.push((ind + off, mode, doc));
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

    #[inline]
    pub fn render(&self, width:uint) -> String {
        let mut pos = 0u;
        let mut cmds = vec![];
        let mut result = String::new();

        cmds.push((0, mode::Break, self));

        loop {
            match cmds.pop() {
                None => {
                    break;
                },
                Some((ind, mode, doc)) => match doc {
                    &Nil => {
                    },
                    &Append(box ref ldoc, box ref rdoc) => {
                        cmds.push((ind, mode, rdoc));
                        cmds.push((ind, mode, ldoc));
                    },
                    &Group(box ref doc) => match mode {
                        mode::Flat => {
                            cmds.push((ind, mode::Flat, doc));
                        },
                        mode::Break => {
                            // FIXME: fitting() modifies cmds so we must either
                            // take a clone and pass it or rewrite fitting()
                            // to modify a separate cmd stack and only observe
                            // this one.
                            let mut cmds_dup = cmds.clone();
                            let next = (ind, mode::Flat, doc);
                            cmds_dup.push(next);
                            if fitting(cmds_dup, width as int - pos as int) {
                                cmds.push(next);
                            } else {
                                cmds.push((ind, mode::Break, doc));
                            }
                        }
                    },
                    &Nest(off, box ref doc) => {
                        cmds.push((ind + off, mode, doc));
                    },
                    &Newline => {
                        result.push_str(util::string::nl_spaces(ind).as_slice());
                        pos = ind;
                    },
                    &Text(ref str) => {
                        result.push_str(str.as_slice());
                        pos += str.len();
                    },
                }
            }
        }

        result
    }

    #[inline]
    pub fn text<T:Str>(str:T) -> Doc {
        Text(String::from_str(str.as_slice()))
    }

}
