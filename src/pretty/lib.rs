//! This crate defines a
//! [Wadler-style](http://homepages.inf.ed.ac.uk/wadler/papers/prettier/prettier.pdf)
//! pretty-printing API.

use doc::{
    best,
};
use doc::Doc::{
    Append,
    Group,
    Nest,
    Newline,
    Nil,
    Text,
};
use std::io;
use std::borrow::Cow;

mod doc;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Doc<'a>(doc::Doc<'a>);

impl<'a> Doc<'a> {
    #[inline]
    pub fn nil() -> Doc<'a> {
        Doc(Nil)
    }

    #[inline]
    pub fn append(self, that: Doc<'a>) -> Doc<'a> {
        let Doc(ldoc) = self;
        let Doc(rdoc) = that;
        let res = match ldoc {
            Nil  => rdoc,
            ldoc => match rdoc {
                Nil  => ldoc,
                rdoc => Append(Box::new(ldoc), Box::new(rdoc)),
            }
        };
        Doc(res)
    }

    #[inline]
    pub fn as_string<T: ToString>(t: T) -> Doc<'a> {
        Doc::text(t.to_string())
    }

    #[inline]
    pub fn concat(ds: &[Doc<'a>]) -> Doc<'a> {
        ds.iter().fold(Doc::nil(), |a, b| a.append(b.clone()))
    }

    #[inline]
    pub fn group(self) -> Doc<'a> {
        let Doc(doc) = self;
        Doc(Group(Box::new(doc)))
    }

    #[inline]
    pub fn nest(self, off: usize) -> Doc<'a> {
        let Doc(doc) = self;
        Doc(Nest(off, Box::new(doc)))
    }

    #[inline]
    pub fn newline() -> Doc<'a> {
        Doc(Newline)
    }

    #[inline]
    pub fn render<W: io::Write>(&self, width: usize, out: &mut W) -> io::Result<()> {
        let &Doc(ref doc) = self;
        best(doc, width, out).and_then(|()| out.write_all(b"\n"))
    }

    #[inline]
    pub fn text<T: Into<Cow<'a, str>>>(data: T) -> Doc<'a> {
        Doc(Text(data.into()))
    }
}
