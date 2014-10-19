extern crate pretty;

use pretty::doc::{
    Doc
};

#[deriving(Clone)]
#[deriving(Show)]
pub struct Tree<'a> {
    node:String,
    subtrees:&'a[Tree<'a>]
}

impl<'a> Tree<'a> {
    pub fn tree(node:&str, subtrees:&'a[Tree<'a>]) -> Tree<'a> {
        Tree {
            node: String::from_str(node),
            subtrees: subtrees
        }
    }

    fn pretty_trees(trees:&'a[Tree<'a>]) -> Doc {
        match trees {
            [] => fail!(),
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
        Doc::text(self.node.clone()).append(Tree::pretty_bracket(self.subtrees))
    }
}

#[allow(dead_code)]
pub fn main() {
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

    print!("{}", example.pretty().to_string(70));
}
