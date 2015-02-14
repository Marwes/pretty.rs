#![feature(io)]

extern crate pretty;

use pretty::{
    Doc
};
use std::old_io as io;
use std::str;

#[derive(Clone)]
#[derive(Debug)]
pub struct Tree<'a> {
    node:String,
    subtrees:&'a[Tree<'a>]
}

impl<'a> Tree<'a> {
    pub fn new(node:&str, subtrees:&'a[Tree<'a>]) -> Tree<'a> {
        Tree {
            node: node.to_string(),
            subtrees: subtrees
        }
    }

    fn pretty_trees(trees:&'a[Tree<'a>]) -> Doc {
        match trees {
            [] => panic!(),
            [ref t] => t.pretty(),
            [ref t, ref ts..] => {
                t.pretty().append(
                    Doc::text(",")
                ).append(
                    Doc::newline()
                ).append(
                    Tree::pretty_trees(*ts)
                )
            }
        }
    }

    fn pretty_bracket(ts:&'a[Tree<'a>]) -> Doc {
        match ts {
            [] => Doc::nil(),
            ts => {
                Doc::text("[").append(
                    Doc::newline().append(
                        Tree::pretty_trees(ts)
                    ).nest(2)
                ).append(
                    Doc::newline()
                ).append(
                    Doc::text("]")
                )
            }
        }
    }

    pub fn pretty(&self) -> Doc {
        Doc::text(
            self.node.clone()
        ).append(
            Tree::pretty_bracket(
                self.subtrees
            )
        ).group()
    }
}

#[allow(dead_code)]
pub fn main() {
    let bbbbbbs =
        [ Tree::new("ccc", &[])
        , Tree::new("dd", &[])
        ];
    let ffffs =
        [ Tree::new("gg", &[])
        , Tree::new("hhh", &[])
        , Tree::new("ii", &[])
        ];
    let aaas =
        [ Tree::new("bbbbbb", &bbbbbbs)
        , Tree::new("eee", &[])
        , Tree::new("ffff", &ffffs)
        ];
    let example = Tree::new("aaaa", &aaas);

    {
        print!("\nwriting to stdout directly:\n");
        let mut out = io::stdout();
        example.pretty().render(70, &mut out)
    }.and_then(|()| {
        print!("\nwriting to string then printing:\n");
        let mut out = io::MemWriter::new();
        example.pretty().render(70, &mut out)
            .map(|()| {
                println!(
                    "{}",
                    str::from_utf8(
                        out.clone().get_ref()
                    ).unwrap_or("<buffer is not a utf-8 encoded string>")
                )
            })
    }).unwrap_or_else(|err| {
        println!("error: {}", err)
    });
}
