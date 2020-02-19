#![feature(nll)]

struct Foo<'a, 'b: 'a>(&'a &'b ());

fn xmute<'a, 'b>(a: &'b ()) -> &'a () {
    unreachable!()
}

pub fn foo<'a, 'b>(u: &'b ()) -> &'a () {
    xmute::<'a, 'b>(u) //~ ERROR lifetime may not live long enough
}

fn main() {}
