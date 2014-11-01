#![crate_name="pretty"]
#![crate_type="rlib"]

use doc::{
    Append,
    Group,
    Nest,
    Newline,
    Nil,
    Text,
    best,
};

mod doc;
mod mode;
mod util;

#[deriving(Clone)]
#[deriving(Show)]
pub struct Doc(doc::Doc);

impl Doc {

    #[inline]
    pub fn nil() -> Doc {
        Doc(Nil)
    }

    #[inline]
    pub fn append(self, that:Doc) -> Doc {
        let Doc(ldoc) = self;
        let Doc(rdoc) = that;
        let res = match ldoc {
            Nil => {
                rdoc
            },
            ldoc => match rdoc {
                Nil => {
                    ldoc
                },
                rdoc => {
                    Append(box ldoc, box rdoc)
                },
            }
        };
        Doc(res)
    }

    #[inline]
    pub fn as_string<T:ToString>(t:T) -> Doc {
        Doc::text(t.to_string())
    }

    #[inline]
    pub fn concat(ds:&[Doc]) -> Doc {
        ds.iter().fold(Doc::nil(), |a, b| a.append(b.clone()))
    }

    #[inline]
    pub fn group(self) -> Doc {
        let Doc(doc) = self;
        Doc(Group(box doc))
    }

    #[inline]
    pub fn nest(self, off:uint) -> Doc {
        let Doc(doc) = self;
        Doc(Nest(off, box doc))
    }

    #[inline]
    pub fn newline() -> Doc {
        Doc(Newline)
    }

    #[inline]
    pub fn render(&self, width:uint) -> String {
        let &Doc(ref doc) = self;
        best(doc, width)
    }

    #[inline]
    pub fn text<T:Str>(str:T) -> Doc {
        Doc(Text(String::from_str(str.as_slice())))
    }

}
