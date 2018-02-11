use std::borrow::Cow;
use std::cmp;
use std::fmt;
use std::io;
use std::ops::Deref;

use termcolor::{ColorSpec, WriteColor};

pub use self::Doc::{Annotated, Append, Group, Nest, Newline, Nil, Space, Text};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Mode {
    Break,
    Flat,
}

/// The concrete document type. This type is not meant to be used directly. Instead use the static
/// functions on `Doc` or the methods on an `DocAllocator`.
///
/// The `B` parameter is used to abstract over pointers to `Doc`. See `RefDoc` and `BoxDoc` for how
/// it is used
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Doc<'a, A, B> {
    Nil,
    Append(B, B),
    Group(B),
    Nest(usize, B),
    Space,
    Newline,
    Text(Cow<'a, str>),
    Annotated(A, B),
}

impl<'a, A, B, S> From<S> for Doc<'a, A, B>
where
    S: Into<Cow<'a, str>>,
{
    fn from(s: S) -> Doc<'a, A, B> {
        Doc::Text(s.into())
    }
}

trait Render {
    type Error;

    fn write_str(&mut self, s: &str) -> Result<usize, Self::Error>;
    fn write_str_all(&mut self, s: &str) -> Result<(), Self::Error>;
}

struct IoWrite<W>(W);

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

struct FmtWrite<W>(W);

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

trait RenderAnnotated<A>: Render {
    fn push_annotation(&mut self, annotation: &A) -> Result<(), Self::Error>;
    fn pop_annotation(&mut self) -> Result<(), Self::Error>;
}

impl<A, W> RenderAnnotated<A> for IoWrite<W>
where
    W: io::Write,
{
    fn push_annotation(&mut self, _: &A) -> Result<(), Self::Error> {
        Ok(())
    }
    fn pop_annotation(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<A, W> RenderAnnotated<A> for FmtWrite<W>
where
    W: fmt::Write,
{
    fn push_annotation(&mut self, _: &A) -> Result<(), Self::Error> {
        Ok(())
    }
    fn pop_annotation(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

struct TermColored<W> {
    color_stack: Vec<ColorSpec>,
    writer: W,
}

impl<W> Render for TermColored<W>
where
    W: io::Write,
{
    type Error = io::Error;

    fn write_str(&mut self, s: &str) -> io::Result<usize> {
        self.writer.write(s.as_bytes())
    }

    fn write_str_all(&mut self, s: &str) -> io::Result<()> {
        self.writer.write_all(s.as_bytes())
    }
}

impl<W> RenderAnnotated<ColorSpec> for TermColored<W>
where
    W: WriteColor,
{
    fn push_annotation(&mut self, color: &ColorSpec) -> Result<(), Self::Error> {
        self.color_stack.push(color.clone());
        self.writer.set_color(color)
    }
    fn pop_annotation(&mut self) -> Result<(), Self::Error> {
        self.color_stack.pop();
        match self.color_stack.last() {
            Some(previous) => self.writer.set_color(previous),
            None => self.writer.reset(),
        }
    }
}

pub struct Pretty<'a, A, D>
where
    A: 'a,
    D: 'a,
{
    doc: &'a Doc<'a, A, D>,
    width: usize,
}

impl<'a, A, D> fmt::Display for Pretty<'a, A, D>
where
    D: Deref<Target = Doc<'a, A, D>>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.doc.render_fmt(self.width, f)
    }
}

impl<'a, A, B> Doc<'a, A, B> {
    /// Writes a rendered document to a `std::io::Write` object.
    #[inline]
    pub fn render<'b, W>(&'b self, width: usize, out: &mut W) -> io::Result<()>
    where
        B: Deref<Target = Doc<'b, A, B>>,
        W: ?Sized + io::Write,
    {
        best(self, width, &mut IoWrite(out))
    }

    /// Writes a rendered document to a `std::fmt::Write` object.
    #[inline]
    pub fn render_fmt<'b, W>(&'b self, width: usize, out: &mut W) -> fmt::Result
    where
        B: Deref<Target = Doc<'b, A, B>>,
        W: ?Sized + fmt::Write,
    {
        best(self, width, &mut FmtWrite(out))
    }

    /// Returns a value which implements `std::fmt::Display`
    ///
    /// ```
    /// use pretty::Doc;
    /// let doc = Doc::<(), _>::group(
    ///     Doc::text("hello").append(Doc::space()).append(Doc::text("world"))
    /// );
    /// assert_eq!(format!("{}", doc.pretty(80)), "hello world");
    /// ```
    #[inline]
    pub fn pretty<'b>(&'b self, width: usize) -> Pretty<'b, A, B>
    where
        B: Deref<Target = Doc<'b, A, B>>,
    {
        Pretty {
            doc: self,
            width: width,
        }
    }
}

impl<'a, B> Doc<'a, ColorSpec, B> {
    #[inline]
    pub fn render_colored<'b, W>(&'b self, width: usize, out: W) -> io::Result<()>
    where
        B: Deref<Target = Doc<'b, ColorSpec, B>>,
        W: WriteColor,
    {
        best(
            self,
            width,
            &mut TermColored {
                color_stack: Vec::new(),
                writer: out,
            },
        )
    }
}

type Cmd<'a, A, B> = (usize, Mode, &'a Doc<'a, A, B>);

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
fn fitting<'a, A, B>(
    next: Cmd<'a, A, B>,
    bcmds: &Vec<Cmd<'a, A, B>>,
    fcmds: &mut Vec<Cmd<'a, A, B>>,
    mut rem: isize,
) -> bool
where
    B: Deref<Target = Doc<'a, A, B>>,
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
                    &Annotated(_, ref doc) => fcmds.push((ind, mode, doc)),
                }
            }
        }
    }
    false
}

#[inline]
fn best<'a, W, A, B>(doc: &'a Doc<'a, A, B>, width: usize, out: &mut W) -> Result<(), W::Error>
where
    B: Deref<Target = Doc<'a, A, B>>,
    W: ?Sized + RenderAnnotated<A>,
{
    let mut pos = 0usize;
    let mut bcmds = vec![(0usize, Mode::Break, doc)];
    let mut fcmds = vec![];
    let mut annotation_levels = vec![];

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
            &Annotated(ref ann, ref doc) => {
                try!(out.push_annotation(ann));
                annotation_levels.push(bcmds.len());
                bcmds.push((ind, mode, doc));
            }
        }

        if annotation_levels.last() == Some(&bcmds.len()) {
            annotation_levels.pop();
            try!(out.pop_annotation());
        }
    }
    Ok(())
}
