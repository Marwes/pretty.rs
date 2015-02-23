#![feature(old_io)]

extern crate pretty;

use pretty::{
    Doc
};
use std::old_io as io;
use std::str;

#[derive(Clone, Debug)]
pub struct Forest<'a>(&'a [Tree<'a>]);

impl<'a> Forest<'a> {
    fn forest(forest: &'a [Tree<'a>]) -> Forest<'a> {
        Forest(forest)
    }

    fn nil() -> Forest<'a> {
        Forest(&[])
    }

    fn bracket(&self) -> Doc {
        if (self.0).len() == 0 {
            Doc::nil()
        } else {
            Doc::text("[")
                .append(
                    Doc::newline()
                        .append(self.pretty())
                        .nest(2))
                .append(Doc::newline())
                .append(Doc::text("]"))
        }
    }

    fn pretty(&self) -> Doc {
        let forest = self.0;
        let mut doc = Doc::nil();
        let mut i = 0;
        let k = forest.len() - 1;
        loop {
            if i < k {
                doc = doc
                    .append(forest[i].pretty()
                            .append(Doc::text(","))
                            .append(Doc::newline()));
            }
            else if i == k {
                doc = doc
                    .append(forest[i].pretty());
                break
            }
            i += 1;
        }
        doc
    }
}

#[derive(Clone, Debug)]
pub struct Tree<'a> {
    node: String,
    forest: Forest<'a>,
}

impl<'a> Tree<'a> {
    pub fn node(node: &str) -> Tree<'a> {
        Tree {
            node: node.to_string(),
            forest: Forest::nil(),
        }
    }

    pub fn node_with_forest(node: &str, forest: &'a [Tree<'a>]) -> Tree<'a> {
        Tree {
            node: node.to_string(),
            forest: Forest::forest(forest),
        }
    }

    pub fn pretty(&self) -> Doc {
        Doc::text(self.node.clone())
            .append((self.forest).bracket())
            .group()
    }
}

#[allow(dead_code)]
pub fn main() {
    let bbbbbbs = [
        Tree::node("ccc"),
        Tree::node("dd"),
    ];
    let ffffs = [
        Tree::node("gg"),
        Tree::node("hhh"),
        Tree::node("ii"),
    ];
    let aaas = [
        Tree::node_with_forest("bbbbbb", &bbbbbbs),
        Tree::node("eee"),
        Tree::node_with_forest("ffff", &ffffs),
    ];
    let example = Tree::node_with_forest("aaaa", &aaas);

    let err_msg = "<buffer is not a utf-8 encoded string>";

    // try writing to stdout
    {
        print!("\nwriting to stdout directly:\n");
        let mut out = io::stdout();
        example
            .pretty()
            .render(70, &mut out)
    // try writing to memory
    }.and_then(|()| {
        print!("\nwriting to string then printing:\n");
        let mut mem = io::MemWriter::new();
        example
            .pretty()
            .render(70, &mut mem)
            // print to console from memory
            .map(|()| {
                let res = str::from_utf8(mem.get_ref()).unwrap_or(err_msg);
                println!("{}", res)
            })
    // print an error if anything failed
    }).unwrap_or_else(|err| {
        println!("error: {}", err)
    });
}
