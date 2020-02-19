#![feature(nll)]

// dummy definition of vec since this doesn't really require Vec
struct Vec<T>(T);

fn main() {
    let mut v: Vec<()> = Vec::<()>(());
    || &mut v;
//~^ ERROR captured variable cannot escape `FnMut` closure body
}
