//! This crate defines a
//! [Wadler-style](http://homepages.inf.ed.ac.uk/wadler/papers/prettier/prettier.pdf)
//! pretty-printing API.
//!
//! Start with with the static functions of [Doc](enum.Doc.html).
//!
//! ## Quick start
//!
//! Let's pretty-print simple sexps!  We want to pretty print sexps like
//!
//! ```lisp
//! (1 2 3)
//! ```
//! or, if the line would be too long, like
//!
//! ```lisp
//! ((1)
//!  (2 3)
//!  (4 5 6))
//! ```
//!
//! A _simple symbolic expression_ consists of a numeric _atom_ or a nested ordered _list_ of
//! symbolic expression children.
//!
//! ```rust
//! # extern crate pretty;
//! # use pretty::*;
//! enum SExp {
//!     Atom(u32),
//!     List(Vec<SExp>),
//! }
//! use SExp::*;
//! # fn main() { }
//! ```
//!
//! We define a simple conversion to a [Doc](enum.Doc.html).  Atoms are rendered as strings; lists
//! are recursively rendered, with spaces between children where appropriate.  Children are
//! [nested]() and [grouped](), allowing them to be laid out in a single line as appropriate.
//!
//! ```rust
//! # extern crate pretty;
//! # use pretty::*;
//! # enum SExp {
//! #     Atom(u32),
//! #     List(Vec<SExp>),
//! # }
//! # use SExp::*;
//! impl SExp {
//!     /// Return a pretty printed format of self.
//!     pub fn to_doc(&self) -> Doc<BoxDoc> {
//!         match self {
//!             &Atom(x) => Doc::as_string(x),
//!             &List(ref xs) =>
//!                 Doc::text("(")
//!                     .append(Doc::intersperse(xs.into_iter().map(|x| x.to_doc()), Doc::space()).nest(1).group())
//!                     .append(Doc::text(")"))
//!         }
//!     }
//! }
//! # fn main() { }
//! ```
//!
//! Next, we convert the [Doc](enum.Doc.html) to a plain old string.
//!
//! ```rust
//! # extern crate pretty;
//! # use pretty::*;
//! # enum SExp {
//! #     Atom(u32),
//! #     List(Vec<SExp>),
//! # }
//! # use SExp::*;
//! # impl SExp {
//! #     /// Return a pretty printed format of self.
//! #     pub fn to_doc(&self) -> Doc<BoxDoc> {
//! #         match self {
//! #             &Atom(x) => Doc::as_string(x),
//! #             &List(ref xs) =>
//! #                 Doc::text("(")
//! #                     .append(Doc::intersperse(xs.into_iter().map(|x| x.to_doc()), Doc::space()).nest(1).group())
//! #                     .append(Doc::text(")"))
//! #         }
//! #     }
//! # }
//! impl SExp {
//!     pub fn to_pretty(&self, width: usize) -> String {
//!         let mut w = Vec::new();
//!         self.to_doc().render(width, &mut w).unwrap();
//!         String::from_utf8(w).unwrap()
//!     }
//! }
//! # fn main() { }
//! ```
//!
//! And finally we can test that the nesting and grouping behaves as we expected.
//!
//! ```rust
//! # extern crate pretty;
//! # use pretty::*;
//! # enum SExp {
//! #     Atom(u32),
//! #     List(Vec<SExp>),
//! # }
//! # use SExp::*;
//! # impl SExp {
//! #     /// Return a pretty printed format of self.
//! #     pub fn to_doc(&self) -> Doc<BoxDoc> {
//! #         match self {
//! #             &Atom(x) => Doc::as_string(x),
//! #             &List(ref xs) =>
//! #                 Doc::text("(")
//! #                     .append(Doc::intersperse(xs.into_iter().map(|x| x.to_doc()), Doc::space()).nest(1).group())
//! #                     .append(Doc::text(")"))
//! #         }
//! #     }
//! # }
//! # impl SExp {
//! #     pub fn to_pretty(&self, width: usize) -> String {
//! #         let mut w = Vec::new();
//! #         self.to_doc().render(width, &mut w).unwrap();
//! #         String::from_utf8(w).unwrap()
//! #     }
//! # }
//! # fn main() {
//! let atom = SExp::Atom(5);
//! assert_eq!("5", atom.to_pretty(10));
//! let list = SExp::List(vec![SExp::Atom(1), SExp::Atom(2), SExp::Atom(3)]);
//! assert_eq!("(1 2 3)", list.to_pretty(10));
//! assert_eq!("\
//!(1
//! 2
//! 3)", list.to_pretty(5));
//! # }
//! ```
//!
//! ## Advanced usage
//!
//! There's a more efficient pattern that uses the [DocAllocator](trait.DocAllocator.html) trait, as
//! implemented by [BoxAllocator](struct.BoxAllocator.html), to allocate
//! [DocBuilder](struct.DocBuilder.html) instances.  See
//! [examples/trees.rs](https://github.com/freebroccolo/pretty.rs/blob/master/examples/trees.rs#L39)
//! for this approach.

extern crate typed_arena;

use doc::Doc::{Append, Group, Nest, Newline, Nil, Space, Text};
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

/// The `DocBuilder` type allows for convenient appending of documents even for arena allocated
/// documents by storing the arena inline.
#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub struct DocBuilder<'a, A: ?Sized>(pub &'a A, pub doc::Doc<'a, A::Doc>)
where
    A: DocAllocator<'a> + 'a;

impl<'a, A: DocAllocator<'a> + 'a> Clone for DocBuilder<'a, A> {
    fn clone(&self) -> Self {
        DocBuilder(self.0, self.1.clone())
    }
}

impl<'a, A: ?Sized> Into<doc::Doc<'a, A::Doc>> for DocBuilder<'a, A>
where
    A: DocAllocator<'a>,
{
    fn into(self) -> doc::Doc<'a, A::Doc> {
        self.1
    }
}

/// The `DocAllocator` trait abstracts over a type which can allocate (pointers to) `Doc`.
pub trait DocAllocator<'a> {
    type Doc: Deref<Target = doc::Doc<'a, Self::Doc>> + Clone;

    fn alloc(&'a self, doc::Doc<'a, Self::Doc>) -> Self::Doc;

    /// Allocate an empty document.
    #[inline]
    fn nil(&'a self) -> DocBuilder<'a, Self> {
        DocBuilder(self, Nil)
    }

    /// Allocate a single newline.
    #[inline]
    fn newline(&'a self) -> DocBuilder<'a, Self> {
        DocBuilder(self, Newline)
    }

    /// Allocate a single space.
    #[inline]
    fn space(&'a self) -> DocBuilder<'a, Self> {
        DocBuilder(self, Space)
    }

    /// Allocate a document containing the text `t.to_string()`.
    ///
    /// The given text must not contain line breaks.
    #[inline]
    fn as_string<T: ToString>(&'a self, t: T) -> DocBuilder<'a, Self> {
        self.text(t.to_string())
    }

    /// Allocate a document containing the given text.
    ///
    /// The given text must not contain line breaks.
    #[inline]
    fn text<T: Into<Cow<'a, str>>>(&'a self, data: T) -> DocBuilder<'a, Self> {
        let text = data.into();
        debug_assert!(!text.contains(|c: char| c == '\n' || c == '\r'));
        DocBuilder(self, Text(text))
    }

    /// Allocate a document concatenating the given documents.
    #[inline]
    fn concat<I>(&'a self, docs: I) -> DocBuilder<'a, Self>
    where
        I: IntoIterator,
        I::Item: Into<doc::Doc<'a, Self::Doc>>,
    {
        docs.into_iter().fold(self.nil(), |a, b| a.append(b))
    }

    /// Allocate a document that intersperses the given separator `S` between the given documents
    /// `[A, B, C, ..., Z]`, yielding `[A, S, B, S, C, S, ..., S, Z]`.
    ///
    /// Compare [the `intersperse` method from the `itertools` crate](https://docs.rs/itertools/0.5.9/itertools/trait.Itertools.html#method.intersperse).
    #[inline]
    fn intersperse<I, S>(&'a self, docs: I, separator: S) -> DocBuilder<'a, Self>
    where
        I: IntoIterator,
        I::Item: Into<doc::Doc<'a, Self::Doc>>,
        S: Into<doc::Doc<'a, Self::Doc>> + Clone,
    {
        let mut result = self.nil();
        let mut iter = docs.into_iter();
        if let Some(first) = iter.next() {
            result = result.append(first);
        }
        for doc in iter {
            result = result.append(separator.clone());
            result = result.append(doc);
        }
        result
    }
}

impl<'a, 's, A: ?Sized> DocBuilder<'a, A>
where
    A: DocAllocator<'a>,
{
    /// Append the given document after this document.
    #[inline]
    pub fn append<B>(self, that: B) -> DocBuilder<'a, A>
    where
        B: Into<doc::Doc<'a, A::Doc>>,
    {
        let DocBuilder(allocator, this) = self;
        let that = that.into();
        let doc = match (this, that) {
            (Nil, that) => that,
            (this, Nil) => this,
            (this, that) => Append(allocator.alloc(this), allocator.alloc(that)),
        };
        DocBuilder(allocator, doc)
    }

    /// Mark this document as a group.
    ///
    /// Groups are layed out on a single line if possible.  Within a group, all basic documents with
    /// several possible layouts are assigned the same layout, that is, they are all layed out
    /// horizontally and combined into a one single line, or they are each layed out on their own
    /// line.
    #[inline]
    pub fn group(self) -> DocBuilder<'a, A> {
        let DocBuilder(allocator, this) = self;
        DocBuilder(allocator, Group(allocator.alloc(this)))
    }

    /// Increase the indentation level of this document.
    #[inline]
    pub fn nest(self, offset: usize) -> DocBuilder<'a, A> {
        if offset == 0 {
            return self;
        }
        let DocBuilder(allocator, this) = self;
        DocBuilder(allocator, Nest(offset, allocator.alloc(this)))
    }
}

/// Newtype wrapper for `&doc::Doc`
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

/// An arena which can be used to allocate `Doc` values.
pub type Arena<'a> = typed_arena::Arena<doc::Doc<'a, RefDoc<'a>>>;

impl<'a, A> DocAllocator<'a> for &'a A
where
    A: ?Sized + DocAllocator<'a>,
{
    type Doc = A::Doc;

    #[inline]
    fn alloc(&'a self, doc: doc::Doc<'a, Self::Doc>) -> Self::Doc {
        (**self).alloc(doc)
    }
}

impl<'a> DocAllocator<'a> for Arena<'a> {
    type Doc = RefDoc<'a>;

    #[inline]
    fn alloc(&'a self, doc: doc::Doc<'a, Self::Doc>) -> Self::Doc {
        static SPACE: doc::Doc<'static, RefDoc<'static>> = Doc::Space;
        static NEWLINE: doc::Doc<'static, RefDoc<'static>> = Doc::Newline;

        RefDoc(match doc {
            Space => &SPACE,
            Newline => &NEWLINE,
            _ => Arena::alloc(self, doc),
        })
    }
}

pub struct BoxAllocator;

static BOX_ALLOCATOR: BoxAllocator = BoxAllocator;

impl<'a> DocAllocator<'a> for BoxAllocator {
    type Doc = BoxDoc<'a>;

    #[inline]
    fn alloc(&'a self, doc: doc::Doc<'a, Self::Doc>) -> Self::Doc {
        BoxDoc::new(doc)
    }
}

pub use doc::Doc;

impl<'a, B> Doc<'a, B> {
    /// An empty document.
    #[inline]
    pub fn nil() -> Doc<'a, B> {
        Nil
    }

    /// The text `t.to_string()`.
    ///
    /// The given text must not contain line breaks.
    #[inline]
    pub fn as_string<T: ToString>(t: T) -> Doc<'a, B> {
        Doc::text(t.to_string())
    }

    /// A single newline.
    #[inline]
    pub fn newline() -> Doc<'a, B> {
        Newline
    }

    /// The given text, which must not contain line breaks.
    #[inline]
    pub fn text<T: Into<Cow<'a, str>>>(data: T) -> Doc<'a, B> {
        let text = data.into();
        debug_assert!(!text.contains(|c: char| c == '\n' || c == '\r'));
        Text(text)
    }

    /// A space.
    #[inline]
    pub fn space() -> Doc<'a, B> {
        Space
    }
}

impl<'a> Doc<'a, BoxDoc<'a>> {
    /// Append the given document after this document.
    #[inline]
    pub fn append(self, that: Doc<'a, BoxDoc<'a>>) -> Doc<'a, BoxDoc<'a>> {
        DocBuilder(&BOX_ALLOCATOR, self).append(that).into()
    }

    /// A single document concatenating all the given documents.
    #[inline]
    pub fn concat<I>(docs: I) -> Doc<'a, BoxDoc<'a>>
    where
        I: IntoIterator<Item = Doc<'a, BoxDoc<'a>>>,
    {
        docs.into_iter().fold(Doc::nil(), |a, b| a.append(b))
    }

    /// A single document interspersing the given separator `S` between the given documents.  For
    /// example, if the documents are `[A, B, C, ..., Z]`, this yields `[A, S, B, S, C, S, ..., S, Z]`.
    ///
    /// Compare [the `intersperse` method from the `itertools` crate](https://docs.rs/itertools/0.5.9/itertools/trait.Itertools.html#method.intersperse).
    #[inline]
    pub fn intersperse<I, S>(docs: I, separator: S) -> Doc<'a, BoxDoc<'a>>
    where
        I: IntoIterator<Item = Doc<'a, BoxDoc<'a>>>,
        S: Into<Doc<'a, BoxDoc<'a>>> + Clone,
    {
        let separator = separator.into();
        let mut result = Doc::nil();
        let mut iter = docs.into_iter();
        if let Some(first) = iter.next() {
            result = result.append(first);
        }
        for doc in iter {
            result = result.append(separator.clone());
            result = result.append(doc);
        }
        result
    }

    /// Mark this document as a group.
    ///
    /// Groups are layed out on a single line if possible.  Within a group, all basic documents with
    /// several possible layouts are assigned the same layout, that is, they are all layed out
    /// horizontally and combined into a one single line, or they are each layed out on their own
    /// line.
    #[inline]
    pub fn group(self) -> Doc<'a, BoxDoc<'a>> {
        DocBuilder(&BOX_ALLOCATOR, self).group().into()
    }

    /// Increase the indentation level of this document.
    #[inline]
    pub fn nest(self, offset: usize) -> Doc<'a, BoxDoc<'a>> {
        DocBuilder(&BOX_ALLOCATOR, self).nest(offset).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test {
        ($size: expr, $actual: expr, $expected: expr) => {
            let mut s = String::new();
            $actual.render_fmt($size, &mut s).unwrap();
            assert_eq!(s, $expected);
        };
        ($actual: expr, $expected: expr) => {
            test!(70, $actual, $expected)
        }
    }

    #[test]
    fn box_doc_inference() {
        let doc = Doc::group(
            Doc::text("test")
                .append(Doc::space())
                .append(Doc::text("test")),
        );

        test!(doc, "test test");
    }

    #[test]
    fn forced_newline() {
        let doc = Doc::group(
            Doc::text("test")
                .append(Doc::newline())
                .append(Doc::text("test")),
        );

        test!(doc, "test\ntest");
    }

    #[test]
    fn space_do_not_reset_pos() {
        let doc = Doc::group(Doc::text("test").append(Doc::space()))
            .append(Doc::text("test"))
            .append(Doc::group(Doc::space()).append(Doc::text("test")));

        test!(9, doc, "test test\ntest");
    }

    // Tests that the `Doc::newline()` does not cause the rest of document to think that it fits on
    // a single line but instead breaks on the `Doc::space()` to fit with 6 columns
    #[test]
    fn newline_does_not_cause_next_line_to_be_to_long() {
        let doc = Doc::group(
            Doc::text("test").append(Doc::newline()).append(
                Doc::text("test")
                    .append(Doc::space())
                    .append(Doc::text("test")),
            ),
        );

        test!(6, doc, "test\ntest\ntest");
    }

    #[test]
    fn block() {
        let doc = Doc::group(
            Doc::text("{")
                .append(
                    Doc::space()
                        .append(Doc::text("test"))
                        .append(Doc::space())
                        .append(Doc::text("test"))
                        .nest(2),
                )
                .append(Doc::space())
                .append(Doc::text("}")),
        );

        test!(5, doc, "{\n  test\n  test\n}");
    }
}
