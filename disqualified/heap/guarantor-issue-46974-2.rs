// Test that NLL analysis propagates lifetimes correctly through
// field accesses, Box accesses, etc.

#![feature(nll)]

fn bar(s: &Box<(i32,)>) -> &'static i32 {
    // FIXME(#46983): error message should be better
    &s.0 //~ ERROR explicit lifetime required in the type of `s` [E0621]
}

fn main() {
    bar(&Box::new((1,)));
}
