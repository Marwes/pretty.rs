//! This crate defines a
//! [Wadler-style](http://homepages.inf.ed.ac.uk/wadler/papers/prettier/prettier.pdf)
//! pretty-printing API.
extern crate typed_arena;

use doc::Doc::{
    Append,
    Group,
    Nest,
    Newline,
    Nil,
    Text,
};
use std::borrow::Cow;
use std::fmt;
use std::ops::Deref;

mod doc;

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct BoxDoc<'a>(Box<doc::Doc<'a, BoxDoc<'a>>>);

impl<'a> fmt::Debug for BoxDoc<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

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
    fn concat<I>(&'a self, docs: I) -> DocBuilder<'a, Self>
    where I: IntoIterator<Item = doc::Doc<'a, Self::Doc>>
    {
        docs.into_iter().fold(self.nil(), |a, b| a.append(b))
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

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct RefDoc<'a>(&'a doc::Doc<'a, RefDoc<'a>>);

impl<'a> fmt::Debug for RefDoc<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<'a> Deref for RefDoc<'a> {
    type Target = doc::Doc<'a, RefDoc<'a>>;
    
    fn deref(&self) -> &doc::Doc<'a, RefDoc<'a>> {
        &self.0
    }
}

pub type Arena<'a> = typed_arena::Arena<doc::Doc<'a, RefDoc<'a>>>;


impl<'a> Allocator<'a> for Arena<'a> {
    type Doc = RefDoc<'a>;
    
    fn alloc(&'a self, doc: doc::Doc<'a, Self::Doc>) -> Self::Doc {
        RefDoc(Arena::alloc(self, doc))
    }
}

pub struct BoxAllocator;

static BOX_ALLOCATOR: BoxAllocator = BoxAllocator;

impl<'a> Allocator<'a> for BoxAllocator {
    type Doc = BoxDoc<'a>;
    
    fn alloc(&'a self, doc: doc::Doc<'a, Self::Doc>) -> Self::Doc {
        BoxDoc::new(doc)
    }
}

pub type Doc<'a> = doc::Doc<'a, BoxDoc<'a>>;

impl<'a> Doc<'a> {
    #[inline]
    pub fn nil() -> Doc<'a> {
        Nil
    }

    #[inline]
    pub fn append(self, that: Doc<'a>) -> Doc<'a> {
        DocBuilder(&BOX_ALLOCATOR, self).append(that).into()
    }

    #[inline]
    pub fn as_string<T: ToString>(t: T) -> Doc<'a> {
        Doc::text(t.to_string())
    }

    #[inline]
    pub fn concat<I>(&'a self, docs: I) -> Doc<'a>
    where I: IntoIterator<Item = Doc<'a>>
    {
        docs.into_iter().fold(Doc::nil(), |a, b| a.append(b))
    }

    #[inline]
    pub fn group(self) -> Doc<'a> {
        DocBuilder(&BOX_ALLOCATOR, self).group().into()
    }

    #[inline]
    pub fn nest(self, offset: usize) -> Doc<'a> {
        DocBuilder(&BOX_ALLOCATOR, self).nest(offset).into()
    }

    #[inline]
    pub fn newline() -> Doc<'a> {
        Newline
    }

    #[inline]
    pub fn text<T: Into<Cow<'a, str>>>(data: T) -> Doc<'a> {
        Text(data.into())
    }
}
