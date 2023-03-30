use super::*;

pub struct Affixes<'doc, D, A>
where
    D: DocAllocator<'doc, A>,
{
    prefix: DocBuilder<'doc, D, A>,
    suffix: DocBuilder<'doc, D, A>,
    nest: bool,
}

impl<'a, D, A> Clone for Affixes<'a, D, A>
where
    A: Clone,
    D: DocAllocator<'a, A> + 'a,
    D::Doc: Clone,
{
    fn clone(&self) -> Self {
        Affixes {
            prefix: self.prefix.clone(),
            suffix: self.suffix.clone(),
            nest: self.nest,
        }
    }
}

impl<'doc, D, A> Affixes<'doc, D, A>
where
    D: DocAllocator<'doc, A>,
{
    pub fn new(prefix: DocBuilder<'doc, D, A>, suffix: DocBuilder<'doc, D, A>) -> Self {
        Affixes {
            prefix,
            suffix,
            nest: false,
        }
    }

    pub fn nest(mut self) -> Self {
        self.nest = true;
        self
    }
}

/// Formats a set of `prefix` and `suffix` documents around a body
pub struct BlockDoc<'doc, D, A>
where
    D: DocAllocator<'doc, A>,
{
    pub affixes: Vec<Affixes<'doc, D, A>>,
    pub body: DocBuilder<'doc, D, A>,
}

impl<'doc, D, A> BlockDoc<'doc, D, A>
where
    D: DocAllocator<'doc, A>,
    D::Doc: Clone,
    A: Clone,
{
    pub fn format(mut self, nest: isize) -> DocBuilder<'doc, D, A> {
        let arena = self.body.0;

        self.affixes.reverse();
        let fail_on_multi_line = arena.fail().flat_alt(arena.nil());

        (1..self.affixes.len() + 1)
            .rev()
            .map(|split| {
                let (before, after) = self.affixes.split_at(split);
                let last = before.len() == 1;
                docs![
                    arena,
                    docs![
                        arena,
                        arena.concat(before.iter().map(|affixes| affixes.prefix.clone())),
                        if last {
                            arena.nil()
                        } else {
                            fail_on_multi_line.clone()
                        }
                    ]
                    .group(),
                    docs![
                        arena,
                        after.iter().rev().cloned().fold(
                            docs![
                                arena,
                                self.body.clone(),
                                // If there is no prefix then we must not allow the body to laid out on multiple
                                // lines without nesting
                                if !last
                                    && before
                                        .iter()
                                        .all(|affixes| matches!(&*affixes.prefix.1, Doc::Nil))
                                {
                                    fail_on_multi_line.clone()
                                } else {
                                    arena.nil()
                                },
                            ]
                            .nest(nest)
                            .append(
                                arena.concat(after.iter().map(|affixes| affixes.suffix.clone()))
                            ),
                            |acc, affixes| {
                                let mut doc = affixes.prefix.append(acc);
                                if affixes.nest {
                                    doc = doc.nest(nest);
                                }
                                doc.group()
                            },
                        ),
                        arena.concat(before.iter().map(|affixes| affixes.suffix.clone())),
                    ]
                    .group(),
                ]
            })
            .fold(None::<DocBuilder<_, _>>, |acc, doc| {
                Some(match acc {
                    None => doc,
                    Some(acc) => acc.union(doc),
                })
            })
            .unwrap_or(self.body)
    }
}
