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
        [ Tree::new("ccc", [])
        , Tree::new("dd", [])
        ];
    let ffffs =
        [ Tree::new("gg", [])
        , Tree::new("hhh", [])
        , Tree::new("ii", [])
        ];
    let aaas =
        [ Tree::new("bbbbbb", bbbbbbs)
        , Tree::new("eee", [])
        , Tree::new("ffff", ffffs)
        ];
    let example = Tree::new("aaa", aaas);
    let task = || {
        for _ in range (0u, 100u) {
            example.pretty().render(70);
        }
    };
    b.iter(task);
}
