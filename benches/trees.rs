use std::io;

use criterion::{criterion_group, criterion_main, Bencher, Criterion};

use crate::trees::Tree;
use pretty::{Arena, BoxAllocator};

#[path = "../examples/trees.rs"]
mod trees;

macro_rules! bench_trees {
    ($b:expr, $out:expr, $allocator:expr, $size:expr) => {{
        let arena = typed_arena::Arena::new();
        let b = $b;
        let mut out = $out;
        let size = $size;

        let mut example = Tree::node("aaaaaaaaaaaaaaaaaaaaaaaaaaaaa");

        for _ in 0..size {
            let bbbbbbs = arena.alloc_extend([example, Tree::node("dd")].iter().cloned());

            let ffffs = arena.alloc_extend(
                [Tree::node("gg"), Tree::node("hhh"), Tree::node("ii")]
                    .iter()
                    .cloned(),
            );

            let aaas = arena.alloc_extend(
                [
                    Tree::node_with_forest("bbbbbb", bbbbbbs),
                    Tree::node("eee"),
                    Tree::node_with_forest("ffff", ffffs),
                ]
                .iter()
                .cloned(),
            );

            example = Tree::node_with_forest("aaa", aaas);
        }

        let allocator = $allocator;

        b.iter(|| {
            example
                .pretty::<_, ()>(&allocator)
                .1
                .render(70, &mut out)
                .unwrap();
        });
    }};
}

fn bench_sink_box(b: &mut Bencher<'_>) -> () {
    bench_trees!(b, io::sink(), BoxAllocator, 1)
}

fn bench_sink_arena(b: &mut Bencher<'_>) -> () {
    bench_trees!(b, io::sink(), Arena::new(), 1)
}

fn bench_vec_box(b: &mut Bencher<'_>) -> () {
    bench_trees!(b, Vec::new(), BoxAllocator, 1)
}

fn bench_vec_arena(b: &mut Bencher<'_>) -> () {
    bench_trees!(b, Vec::new(), Arena::new(), 1)
}

fn bench_io_box(b: &mut Bencher<'_>) -> () {
    let out = tempfile::tempfile().unwrap();
    bench_trees!(b, io::BufWriter::new(out), BoxAllocator, 1)
}

fn bench_io_arena(b: &mut Bencher<'_>) -> () {
    let out = tempfile::tempfile().unwrap();
    bench_trees!(b, io::BufWriter::new(out), Arena::new(), 1)
}

fn bench_large_sink_box(b: &mut Bencher<'_>) -> () {
    bench_trees!(b, io::sink(), BoxAllocator, 50)
}

fn bench_large_sink_arena(b: &mut Bencher<'_>) -> () {
    bench_trees!(b, io::sink(), Arena::new(), 50)
}

fn bench_large_vec_box(b: &mut Bencher<'_>) -> () {
    bench_trees!(b, Vec::new(), BoxAllocator, 50)
}

fn bench_large_vec_arena(b: &mut Bencher<'_>) -> () {
    bench_trees!(b, Vec::new(), Arena::new(), 50)
}

fn bench_large_io_box(b: &mut Bencher<'_>) -> () {
    let out = tempfile::tempfile().unwrap();
    bench_trees!(b, io::BufWriter::new(out), BoxAllocator, 50)
}

fn bench_large_io_arena(b: &mut Bencher<'_>) -> () {
    let out = tempfile::tempfile().unwrap();
    bench_trees!(b, io::BufWriter::new(out), Arena::new(), 50)
}

fn bench_pretty(c: &mut Criterion) {
    {
        let mut group = c.benchmark_group("small");
        group.bench_function("sink_box", bench_sink_box);
        group.bench_function("sink_arena", bench_sink_arena);
        group.bench_function("vec_box", bench_vec_box);
        group.bench_function("vec_arena", bench_vec_arena);
        group.bench_function("io_box", bench_io_box);
        group.bench_function("io_arena", bench_io_arena);
    }

    {
        let mut group = c.benchmark_group("large");
        group.bench_function("sink_box", bench_large_sink_box);
        group.bench_function("sink_arena", bench_large_sink_arena);
        group.bench_function("vec_box", bench_large_vec_box);
        group.bench_function("vec_arena", bench_large_vec_arena);
        group.bench_function("io_box", bench_large_io_box);
        group.bench_function("io_arena", bench_large_io_arena);
    }
}

criterion_group!(benches, bench_pretty);
criterion_main!(benches);
