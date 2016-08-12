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
use std::ops::Deref;

mod doc;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Doc<'a>(Box<doc::Doc<'a, Doc<'a>>>);

impl<'a> Deref for Doc<'a> {
    type Target = doc::Doc<'a, Doc<'a>>;
    
    fn deref(&self) -> &doc::Doc<'a, Doc<'a>> {
        &self.0
    }
}

impl<'a> Doc<'a> {
    #[inline]
    pub fn nil() -> Doc<'a> {
        Doc(Box::new(Nil))
    }

    #[inline]
    pub fn append(self, that: Doc<'a>) -> Doc<'a> {
        match &*self {
            &Nil  => that,
            _ => match &*that {
                &Nil  => self,
                _ => Doc(Box::new(Append(self, that))),
            }
        }
    }

    #[inline]
    pub fn as_string<T: ToString>(t: T) -> Doc<'a> {
        Doc::text(t.to_string())
    }

    #[inline]
    pub fn concat(docs: &[Doc<'a>]) -> Doc<'a> {
        docs.iter().fold(Doc::nil(), |a, b| a.append(b.clone()))
    }

    #[inline]
    pub fn group(self) -> Doc<'a> {
        Doc(Box::new(Group(self)))
    }

    #[inline]
    pub fn nest(self, offset: usize) -> Doc<'a> {
        Doc(Box::new(Nest(offset, self)))
    }

    #[inline]
    pub fn newline() -> Doc<'a> {
        Doc(Box::new(Newline))
    }

    #[inline]
    pub fn render<W: io::Write>(&self, width: usize, out: &mut W) -> io::Result<()> {
        let &Doc(ref doc) = self;
        best(doc, width, out).and_then(|()| out.write_all(b"\n"))
    }

    #[inline]
    pub fn text<T: Into<Cow<'a, str>>>(data: T) -> Doc<'a> {
        Doc(Box::new(Text(data.into())))
    }
}
