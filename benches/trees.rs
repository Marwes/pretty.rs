#![feature(test)]

extern crate test;
extern crate pretty;
extern crate typed_arena;

use trees::{
    Tree,
};

use pretty::BoxAllocator;

use typed_arena::Arena;

#[path="../examples/trees.rs"]
mod trees;

macro_rules! bench_trees {
    ($b: expr, $allocator: expr) => {{
        let b = $b;
        let allocator = $allocator;
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
        let mut out = std::io::sink();
        let task = || {
            example.pretty(&allocator).1.render(70, &mut out).unwrap();
        };
        b.iter(task);
    }}
}

#[bench]
fn bench_box(b: &mut test::Bencher) -> () {
    bench_trees!(b, BoxAllocator)
}
#[bench]
fn bench_arena(b: &mut test::Bencher) -> () {
    bench_trees!(b, Arena::new())
}