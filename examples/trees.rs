extern crate pretty;

use pretty::{
    Doc
};

#[deriving(Clone)]
#[deriving(Show)]
pub struct Tree<'a> {
    node:String,
    subtrees:&'a[Tree<'a>]
}

impl<'a> Tree<'a> {
    pub fn new(node:&str, subtrees:&'a[Tree<'a>]) -> Tree<'a> {
        Tree {
            node: String::from_str(node),
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

    let example = Tree::new("aaaa", aaas);

    print!("{}", example.pretty().render(70));
}
