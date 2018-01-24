use std::cmp;
use std::fmt;
use std::io;
use std::ops::Deref;

pub use Doc::{self, Append, Group, Nest, Newline, Nil, Space, Text};

pub trait Render {
    type Error;

    fn write_str(&mut self, s: &str) -> Result<usize, Self::Error>;
    fn write_str_all(&mut self, s: &str) -> Result<(), Self::Error>;
}

pub struct IoWrite<W>(pub W);

impl<W> Render for IoWrite<W>
where
    W: io::Write,
{
    type Error = io::Error;

    fn write_str(&mut self, s: &str) -> io::Result<usize> {
        self.0.write(s.as_bytes())
    }

    fn write_str_all(&mut self, s: &str) -> io::Result<()> {
        self.0.write_all(s.as_bytes())
    }
}

pub struct FmtWrite<W>(pub W);

impl<W> Render for FmtWrite<W>
where
    W: fmt::Write,
{
    type Error = fmt::Error;

    fn write_str(&mut self, s: &str) -> Result<usize, fmt::Error> {
        self.write_str_all(s).map(|_| s.len())
    }

    fn write_str_all(&mut self, s: &str) -> fmt::Result {
        self.0.write_str(s)
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Mode {
    Break,
    Flat,
}

type Cmd<'a, B> = (usize, Mode, &'a Doc<'a, B>);

fn write_newline<W>(ind: usize, out: &mut W) -> Result<(), W::Error>
where
    W: ?Sized + Render,
{
    try!(out.write_str_all("\n"));
    write_spaces(ind, out)
}

fn write_spaces<W>(spaces: usize, out: &mut W) -> Result<(), W::Error>
where
    W: ?Sized + Render,
{
    macro_rules! make_spaces {
        () => {
            ""
        };
        ($s: tt $($t: tt)*) => {
            concat!("          ", make_spaces!($($t)*))
        };
    }
    const SPACES: &str = make_spaces!(,,,,,,,,,,);
    let mut inserted = 0;
    while inserted < spaces {
        let insert = cmp::min(SPACES.len(), spaces - inserted);
        inserted += try!(out.write_str(&SPACES[..insert]));
    }
    Ok(())
}

#[inline]
fn fitting<'a, B>(
    next: Cmd<'a, B>,
    bcmds: &Vec<Cmd<'a, B>>,
    fcmds: &mut Vec<Cmd<'a, B>>,
    mut rem: isize,
) -> bool
where
    B: Deref<Target = Doc<'a, B>>,
{
    let mut bidx = bcmds.len();
    fcmds.clear(); // clear from previous calls from best
    fcmds.push(next);
    while rem >= 0 {
        match fcmds.pop() {
            None => {
                if bidx == 0 {
                    // All commands have been processed
                    return true;
                } else {
                    fcmds.push(bcmds[bidx - 1]);
                    bidx -= 1;
                }
            }
            Some((ind, mode, doc)) => {
                match doc {
                    &Nil => {}
                    &Append(ref ldoc, ref rdoc) => {
                        fcmds.push((ind, mode, rdoc));
                        // Since appended documents often appear in sequence on the left side we
                        // gain a slight performance increase by batching these pushes (avoiding
                        // to push and directly pop `Append` documents)
                        let mut doc = ldoc;
                        while let Append(ref l, ref r) = **doc {
                            fcmds.push((ind, mode, r));
                            doc = l;
                        }
                        fcmds.push((ind, mode, doc));
                    }
                    &Group(ref doc) => {
                        fcmds.push((ind, mode, doc));
                    }
                    &Nest(off, ref doc) => {
                        fcmds.push((ind + off, mode, doc));
                    }
                    &Space => match mode {
                        Mode::Flat => {
                            rem -= 1;
                        }
                        Mode::Break => {
                            return true;
                        }
                    },
                    &Newline => return true,
                    &Text(ref str) => {
                        rem -= str.len() as isize;
                    }
                }
            }
        }
    }
    false
}

#[inline]
pub fn best<'a, W, B>(doc: &'a Doc<'a, B>, width: usize, out: &mut W) -> Result<(), W::Error>
where
    B: Deref<Target = Doc<'a, B>>,
    W: ?Sized + Render,
{
    let mut pos = 0usize;
    let mut bcmds = vec![(0usize, Mode::Break, doc)];
    let mut fcmds = vec![];
    while let Some((ind, mode, doc)) = bcmds.pop() {
        match doc {
            &Nil => {}
            &Append(ref ldoc, ref rdoc) => {
                bcmds.push((ind, mode, rdoc));
                let mut doc = ldoc;
                while let Append(ref l, ref r) = **doc {
                    bcmds.push((ind, mode, r));
                    doc = l;
                }
                bcmds.push((ind, mode, doc));
            }
            &Group(ref doc) => match mode {
                Mode::Flat => {
                    bcmds.push((ind, Mode::Flat, doc));
                }
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
            }
            &Space => match mode {
                Mode::Flat => {
                    try!(write_spaces(1, out));
                }
                Mode::Break => {
                    try!(write_newline(ind, out));
                    pos = ind;
                }
            },
            &Newline => {
                try!(write_newline(ind, out));
                pos = ind;

                // Since this newline caused an early break we don't know if the remaining
                // documents fit the next line so recalculate if they fit
                fcmds.clear();
                let docs = bcmds.len()
                    - bcmds
                        .iter()
                        .rev()
                        .position(|t| t.1 == Mode::Break)
                        .unwrap_or(bcmds.len());
                fcmds.extend_from_slice(&bcmds[docs..]);
                if let Some(next) = fcmds.pop() {
                    let rem = width as isize - pos as isize;
                    if !fitting(next, &bcmds, &mut fcmds, rem) {
                        for &mut (_, ref mut mode, _) in &mut bcmds[docs..] {
                            *mode = Mode::Break;
                        }
                    }
                }
            }
            &Text(ref s) => {
                try!(out.write_str_all(s));
                pos += s.len();
            }
        }
    }
    Ok(())
}
