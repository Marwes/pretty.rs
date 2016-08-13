//! This crate defines a
//! [Wadler-style](http://homepages.inf.ed.ac.uk/wadler/papers/prettier/prettier.pdf)
//! pretty-printing API.
extern crate typed_arena;

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

use typed_arena::Arena;

mod doc;

pub type Doc<'a, B> = doc::Doc<'a, B>;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct BoxDoc<'a>(Box<doc::Doc<'a, BoxDoc<'a>>>);

impl<'a> BoxDoc<'a> {
    fn new(doc: doc::Doc<'a, BoxDoc<'a>>) -> BoxDoc<'a> {
        BoxDoc(Box::new(doc))
    }
}

impl<'a> Deref for BoxDoc<'a> {
    type Target = doc::Doc<'a, BoxDoc<'a>>;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct DocBuilder<'a, A: ?Sized>(pub &'a A, pub doc::Doc<'a, A::Doc>)
    where A: Allocator<'a> + 'a;

impl <'a, A> Into<doc::Doc<'a, A::Doc>> for DocBuilder<'a, A>
    where A: Allocator<'a>
{
    fn into(self) -> doc::Doc<'a, A::Doc> {
        self.1
    }
}

pub trait Allocator<'a> {
    type Doc: Deref<Target = doc::Doc<'a, Self::Doc>> + Clone;
    fn alloc(&'a self, doc::Doc<'a, Self::Doc>) -> Self::Doc;

    #[inline]
    fn nil(&'a self) -> DocBuilder<'a, Self> {
        DocBuilder(self, Nil)
    }

    #[inline]
    fn newline(&'a self) -> DocBuilder<'a, Self> {
        DocBuilder(self, Newline)
    }

    #[inline]
    fn as_string<T: ToString>(&'a self, t: T) -> DocBuilder<'a, Self> {
        self.text(t.to_string())
    }

    #[inline]
    fn text<T: Into<Cow<'a, str>>>(&'a self, data: T) -> DocBuilder<'a, Self> {
        DocBuilder(self, Text(data.into()))
    }

    #[inline]
    fn concat(&'a self, docs: &[doc::Doc<'a, Self::Doc>]) -> DocBuilder<'a, Self> {
        docs.iter().cloned().fold(self.nil(), |a, b| a.append(b))
    }
}


impl<'a, 's, A: ?Sized> DocBuilder<'a, A> where A: Allocator<'a> {
    #[inline]
    pub fn append<B>(self, that: B) -> DocBuilder<'a, A>
    where B: Into<doc::Doc<'a, A::Doc>>,
    {
        let DocBuilder(allocator, this) = self;
        let that = that.into();
        let doc = match this {
            Nil  => that,
            _ => match that {
                Nil  => this,
                _ => Append(allocator.alloc(this), allocator.alloc(that)),
            }
        };
        DocBuilder(allocator, doc)
    }

    #[inline]
    pub fn group(self) -> DocBuilder<'a, A>
    {
        let DocBuilder(allocator, this) = self;
        DocBuilder(allocator, Group(allocator.alloc(this)))
    }

    #[inline]
    pub fn nest(self, offset: usize) -> DocBuilder<'a, A>
    {
        let DocBuilder(allocator, this) = self;
        DocBuilder(allocator, Nest(offset, allocator.alloc(this)))
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct RefDoc<'a>(&'a doc::Doc<'a, RefDoc<'a>>);

impl<'a> Deref for RefDoc<'a> {
    type Target = doc::Doc<'a, RefDoc<'a>>;
    
    fn deref(&self) -> &doc::Doc<'a, RefDoc<'a>> {
        &self.0
    }
}


impl<'a> Allocator<'a> for Arena<doc::Doc<'a, RefDoc<'a>>> {
    type Doc = RefDoc<'a>;
    
    fn alloc(&'a self, doc: doc::Doc<'a, Self::Doc>) -> Self::Doc {
        RefDoc(Arena::alloc(self, doc))
    }
}

pub struct BoxAllocator;

impl<'a> Allocator<'a> for BoxAllocator {
    type Doc = BoxDoc<'a>;
    
    fn alloc(&'a self, doc: doc::Doc<'a, Self::Doc>) -> Self::Doc {
        BoxDoc::new(doc)
    }
}
impl<'a> BoxDoc<'a> {
    #[inline]
    pub fn nil() -> BoxDoc<'a> {
        BoxDoc::new(Nil)
    }

    #[inline]
    pub fn append(self, that: BoxDoc<'a>) -> BoxDoc<'a> {
        match &*self {
            &Nil  => that,
            _ => match &*that {
                &Nil  => self,
                _ => BoxDoc::new(Append(self, that)),
            }
        }
    }

    #[inline]
    pub fn as_string<T: ToString>(t: T) -> BoxDoc<'a> {
        BoxDoc::text(t.to_string())
    }

    #[inline]
    pub fn concat(docs: &[BoxDoc<'a>]) -> BoxDoc<'a> {
        docs.iter().fold(BoxDoc::nil(), |a, b| a.append(b.clone()))
    }

    #[inline]
    pub fn group(self) -> BoxDoc<'a> {
        BoxDoc::new(Group(self))
    }

    #[inline]
    pub fn nest(self, offset: usize) -> BoxDoc<'a> {
        BoxDoc::new(Nest(offset, self))
    }

    #[inline]
    pub fn newline() -> BoxDoc<'a> {
        BoxDoc::new(Newline)
    }

    #[inline]
    pub fn render<W: io::Write>(&self, width: usize, out: &mut W) -> io::Result<()> {
        let &BoxDoc(ref doc) = self;
        best(doc, width, out).and_then(|()| out.write_all(b"\n"))
    }

    #[inline]
    pub fn text<T: Into<Cow<'a, str>>>(data: T) -> BoxDoc<'a> {
        BoxDoc::new(Text(data.into()))
    }
}
