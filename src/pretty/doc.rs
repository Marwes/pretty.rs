use super::mode::{
    Mode,
};
use super::util;
use std::old_io as io;

pub use self::Doc::{
    Nil,
    Append,
    Group,
    Nest,
    Newline,
    Text,
};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Doc {
    Nil,
    Append(Box<Doc>, Box<Doc>),
    Group(Box<Doc>),
    Nest(usize, Box<Doc>),
    Newline,
    Text(String),
}

type Cmd<'a> = (usize, Mode, &'a Doc);

#[inline]
fn fitting<'a>(
       next: Cmd<'a>,
      bcmds: &Vec<Cmd<'a>>,
      fcmds: &mut Vec<Cmd<'a>>,
    mut rem: isize
) -> bool {
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
                &Append(ref ldoc, ref rdoc) => {
                    fcmds.push((ind, mode, rdoc));
                    fcmds.push((ind, mode, ldoc));
                },
                &Group(ref doc) => {
                    fcmds.push((ind, mode, doc));
                },
                &Nest(off, ref doc) => {
                    fcmds.push((ind + off, mode, doc));
                },
                &Newline => {
                    fits = true;
                },
                &Text(ref str) => {
                    rem -= str.len() as isize;
                },
            }
        }
    }

    fits
}

#[inline]
pub fn best<W: io::Writer>(
      doc: &Doc,
    width: usize,
      out: &mut W
) -> io::IoResult<()> {
    let mut res   = Ok(());
    let mut pos   = 0;
    let mut bcmds = vec![(0, Mode::Break, doc)];
    let mut fcmds = vec![];

    while res.is_ok() {
        match bcmds.pop() {
            None => {
                break;
            },
            Some((ind, mode, doc)) => match doc {
                &Nil => {
                },
                &Append(ref ldoc, ref rdoc) => {
                    bcmds.push((ind, mode, rdoc));
                    bcmds.push((ind, mode, ldoc));
                },
                &Group(ref doc) => match mode {
                    Mode::Flat => {
                        bcmds.push((ind, Mode::Flat, doc));
                    },
                    Mode::Break => {
                        let next = (ind, Mode::Flat, &**doc);
                        let rem  = width as isize - pos as isize;
                        if fitting(next, &bcmds, &mut fcmds, rem) {
                            bcmds.push(next);
                        } else {
                            bcmds.push((ind, Mode::Break, doc));
                        }
                    }
                },
                &Nest(off, ref doc) => {
                    bcmds.push((ind + off, mode, doc));
                },
                &Newline => {
                    res = out.write_str(&util::string::nl_spaces(ind));
                    pos = ind;
                },
                &Text(ref s) => {
                    res = out.write_str(&s);
                    pos += s.len();
                },
            }
        }
    }

    res
}
