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
fn fitting<'a>(next:Cmd<'a>,
               bcmds:&Vec<Cmd<'a>>,
               fcmds:&mut Vec<Cmd<'a>>,
               mut rem:int)
               -> bool {
    let mut bidx = bcmds.len();
    let mut fits = true;

    fcmds.clear();      // clear from previous calls from best
    fcmds.push(next);

    loop {
        if rem < 0 {
            fits = false;
            break;
        }
        match fcmds.pop() {
            None => {
                if bidx == 0 {
                    break;
                } else {
                    fcmds.push(bcmds[ bidx - 1 ]);
                    bidx -= 1;
                }
            },
            Some((ind, mode, doc)) => match doc {
                &Nil => {
                },
                &Append(box ref ldoc, box ref rdoc) => {
                    fcmds.push((ind, mode, rdoc));
                    fcmds.push((ind, mode, ldoc));
                },
                &Group(box ref doc) => {
                    fcmds.push((ind, mode, doc));
                },
                &Nest(off, box ref doc) => {
                    fcmds.push((ind + off, mode, doc));
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

#[inline(always)]
fn best(doc:&Doc, width:uint) -> String {
    let mut pos = 0u;
    let mut result = String::new();

    let mut bcmds = vec![(0, mode::Break, doc)];
    let mut fcmds = vec![];

    loop {
        match bcmds.pop() {
            None => {
                break;
            },
            Some((ind, mode, doc)) => match doc {
                &Nil => {
                },
                &Append(box ref ldoc, box ref rdoc) => {
                    bcmds.push((ind, mode, rdoc));
                    bcmds.push((ind, mode, ldoc));
                },
                &Group(box ref doc) => match mode {
                    mode::Flat => {
                        bcmds.push((ind, mode::Flat, doc));
                    },
                    mode::Break => {
                        let next = (ind, mode::Flat, doc);
                        if fitting(next,
                                   &bcmds,
                                   &mut fcmds,
                                   width as int - pos as int) {
                            bcmds.push(next);
                        } else {
                            bcmds.push((ind, mode::Break, doc));
                        }
                    }
                },
                &Nest(off, box ref doc) => {
                    bcmds.push((ind + off, mode, doc));
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
        best(self, width)
    }

    #[inline]
    pub fn text<T:Str>(str:T) -> Doc {
        Text(String::from_str(str.as_slice()))
    }

}
