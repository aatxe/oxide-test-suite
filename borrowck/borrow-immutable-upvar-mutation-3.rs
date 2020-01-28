#![feature(unboxed_closures)]

// Tests that we can't assign to or mutably borrow upvars from `Fn`
// closures (issue #17780)

fn set<'a>(x: &'a mut usize) { *x = 5; }

fn to_fn(f: fn()) -> fn() { f }

fn main() {
    // By-ref captures
    let mut z: usize = 0;
    let tmp: &'t mut usize = &mut z;
    let _h: fn() = to_fn(|| { set::<'t>(tmp); to_fn(|| z = 42); }); //~ ERROR cannot assign
}
