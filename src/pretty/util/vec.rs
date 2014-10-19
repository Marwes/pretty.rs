pub fn prepend<A:Clone>(mut v: Vec<A>, x:A) -> Vec<A> {
    // let mut res = v;
    v.insert(0, x);
    v
}
