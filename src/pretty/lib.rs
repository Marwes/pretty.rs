#![feature(core, io)]

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
use std::old_io as io;

mod doc;
mod mode;
mod util;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Doc(doc::Doc);

impl Doc {
    #[inline]
    pub fn nil() -> Doc {
        Doc(Nil)
    }

    #[inline]
    pub fn append(self, that: Doc) -> Doc {
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
    pub fn as_string<T: ToString>(t: T) -> Doc {
        Doc::text(t.to_string())
    }

    #[inline]
    pub fn concat(ds: &[Doc]) -> Doc {
        ds.iter().fold(Doc::nil(), |a, b| a.append(b.clone()))
    }

    #[inline]
    pub fn group(self) -> Doc {
        let Doc(doc) = self;
        Doc(Group(Box::new(doc)))
    }

    #[inline]
    pub fn nest(self, off: u64) -> Doc {
        let Doc(doc) = self;
        Doc(Nest(off, Box::new(doc)))
    }

    #[inline]
    pub fn newline() -> Doc {
        Doc(Newline)
    }

    #[inline]
    pub fn render<W: io::Writer>(&self, width: u64, out: &mut W) -> io::IoResult<()> {
        let &Doc(ref doc) = self;
        best(doc, width, out).and_then(|()| out.write_line(""))
    }

    #[inline]
    pub fn text<T: Str>(s: T) -> Doc {
        Doc(Text(s.as_slice().to_string()))
    }
}
