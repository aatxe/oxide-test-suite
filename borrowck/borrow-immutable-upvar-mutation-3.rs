#![feature(unboxed_closures)]

// Tests that we can't assign to or mutably borrow upvars from `Fn`
// closures (issue #17780)

fn set<'a>(x: &'a mut usize) { *x = 5; }

fn to_fn(f: fn()) -> fn() { f }

fn main() {
    // By-ref captures
    let mut z: usize = 0;
    let tmp0: &'t0 mut usize = &mut z;
    let tmp1: &'t1 mut usize = &mut z;
    let _h: fn() = to_fn(|| { set::<'t0>(tmp); to_fn(|| *tmp1 = 42); }); //~ ERROR cannot assign
}
