#![feature(unboxed_closures)]

// Tests that we can't assign to or mutably borrow upvars from `Fn`
// closures (issue #17780)

fn set<'a>(x: &'a mut usize) { *x = 5; }

fn to_fn(f: fn()) -> fn() { f }

fn main() {
    // By-ref captures
    let mut x: usize = 0;
    let tmp0: &'t0 usize = &x;
    let _f: fn() = to_fn(|| *tmp0 = 42); //~ ERROR cannot assign
}
