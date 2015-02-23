#![feature(old_io)]
#![feature(test)]

#![allow(unused_attributes)]

// external crates
extern crate test;

// local crates
extern crate pretty;

// local mod imports
use trees::{
    Tree,
};
use std::old_io as io;

// custom mod imports
#[path="../examples/trees.rs"]
mod trees;

#[bench]
fn bench(b: &mut test::Bencher) -> () {
    let bbbbbbs =
        [ Tree::node("ccc")
        , Tree::node("dd")
        ];
    let ffffs =
        [ Tree::node("gg")
        , Tree::node("hhh")
        , Tree::node("ii")
        ];
    let aaas =
        [ Tree::node_with_forest("bbbbbb", &bbbbbbs)
        , Tree::node("eee")
        , Tree::node_with_forest("ffff", &ffffs)
        ];
    let example = Tree::node_with_forest("aaa", &aaas);
    let mut out = io::util::NullWriter;
    let task = || {
        example.pretty().render(70, &mut out).unwrap();
    };
    b.iter(task);
}
