use std::cmp;
use std::io;
use std::ops::Deref;

pub use self::Doc::{
    Nil,
    Append,
    Group,
    Nest,
    Newline,
    Text,
};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Mode {
    Break,
    Flat,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Doc<'a, B> {
    Nil,
    Append(B, B),
    Group(B),
    Nest(usize, B),
    Newline,
    Text(::std::borrow::Cow<'a, str>),
}

impl<'a, B> Doc<'a, B> {
    #[inline]
    pub fn render<'b, W: io::Write>(&'b self, width: usize, out: &mut W) -> io::Result<()>
    where B: Deref<Target = Doc<'b, B>>
    {
        best(self, width, out).and_then(|()| out.write_all(b"\n"))
    }
}

type Cmd<'a, B> = (usize, Mode, &'a Doc<'a, B>);

#[inline]
fn fitting<'a, B>(
    next: Cmd<'a, B>,
    bcmds: &Vec<Cmd<'a, B>>,
    fcmds: &mut Vec<Cmd<'a, B>>,
    mut rem: isize,
) -> bool
where B: Deref<Target = Doc<'a, B>>
{
    let mut bidx = bcmds.len();
    let mut fits = true;
    fcmds.clear(); // clear from previous calls from best
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
pub fn best<'a, W: io::Write, B>(
    doc: &'a Doc<'a, B>,
    width: usize,
    out: &mut W,
) -> io::Result<()>
where B: Deref<Target = Doc<'a, B>> 
{
    let mut pos = 0usize;
    let mut bcmds = vec![(0usize, Mode::Break, doc)];
    let mut fcmds = vec![];
    loop {
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
                        let rem = width as isize - pos as isize;
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
                    const SPACES: [u8; 100] = [b' '; 100];
                    try!(out.write_all(b"\n"));
                    let mut inserted = 0;
                    while inserted < ind {
                        let insert = cmp::min(100, ind - inserted);
                        inserted += insert;
                        try!(out.write_all(&SPACES[..insert]));
                    }
                    pos = ind;
                },
                &Text(ref s) => {
                    try!(out.write_all(&s.as_bytes()));
                    pos += s.len();
                },
            }
        }
    }
    Ok(())
}
