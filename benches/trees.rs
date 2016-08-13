#![feature(test)]

extern crate test;
extern crate pretty;
extern crate typed_arena;
extern crate tempfile;

use std::io;

use pretty::BoxAllocator;

use typed_arena::Arena;


use trees::{
    Tree,
};

#[path="../examples/trees.rs"]
mod trees;

macro_rules! bench_trees {
    ($b: expr, $out: expr, $allocator: expr) => {{
        let b = $b;
        let allocator = $allocator;
        let mut out = $out;

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
        let task = || {
            example.pretty(&allocator).1.render(70, &mut out).unwrap();
        };
        b.iter(task);
    }}
}

#[bench]
fn bench_sink_box(b: &mut test::Bencher) -> () {
    bench_trees!(b, io::sink(), BoxAllocator)
}
#[bench]
fn bench_sink_arena(b: &mut test::Bencher) -> () {
    bench_trees!(b, io::sink(), Arena::new())
}

#[bench]
fn bench_vec_box(b: &mut test::Bencher) -> () {
    bench_trees!(b, Vec::new(), BoxAllocator)
}
#[bench]
fn bench_vec_arena(b: &mut test::Bencher) -> () {
    bench_trees!(b, Vec::new(), Arena::new())
}

#[bench]
fn bench_io_box(b: &mut test::Bencher) -> () {
    let out = tempfile::tempfile().unwrap();
    bench_trees!(b, io::BufWriter::new(out), BoxAllocator)
}
#[bench]
fn bench_io_arena(b: &mut test::Bencher) -> () {
    let out = tempfile::tempfile().unwrap();
    bench_trees!(b, io::BufWriter::new(out), Arena::new())
}