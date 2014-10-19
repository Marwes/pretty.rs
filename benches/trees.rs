// external crates
extern crate test;

// local crates
extern crate pretty;

// local mod imports
use trees::{
    Tree,
};

// custom mod imports
#[path="../examples/trees.rs"]
mod trees;

#[bench]
fn bench(b:&mut test::Bencher) -> () {
    let bbbbbbs =
        [ Tree::tree("ccc", [])
        , Tree::tree("dd", [])
        ];
    let ffffs =
        [ Tree::tree("gg", [])
        , Tree::tree("hhh", [])
        , Tree::tree("ii", [])
        ];
    let aaas =
        [ Tree::tree("bbbbbb", bbbbbbs)
        , Tree::tree("eee", [])
        , Tree::tree("ffff", ffffs)
        ];
    let example = Tree::tree("aaa", aaas);
    let task = || {
        example.pretty().to_string(70)
    };
    b.iter(task);
}
