#![feature(unboxed_closures)]

// Tests that we can't assign to or mutably borrow upvars from `Fn`
// closures (issue #17780)

fn set<'a>(x: &'a mut usize) { *x = 5; }

fn to_fn(f: fn()) -> fn() { f }

fn main() {
    // By-ref captures
    let mut x: usize = 0;
    let _f: fn() = to_fn(|| x = 42); //~ ERROR cannot assign

    let mut y: usize = 0;
    let tmp: &'t mut usize = &mut y;
    let _g: fn() = to_fn(|| set::<'t>(tmp)); //~ ERROR cannot borrow

    let mut z: usize = 0;
    let tmp: &'t mut usize = &mut z;
    let _h: fn() = to_fn(|| { set::<'t>(tmp); to_fn(|| z = 42); }); //~ ERROR cannot assign

    // By-value captures
    let mut x: usize = 0;
    let _f: fn() = to_fn(move || x = 42); //~ ERROR cannot assign

    let mut y: usize = 0;
    let tmp: &'t mut usize = &mut y;
    let _g: fn() = to_fn(move || set::<'t>(tmp)); //~ ERROR cannot borrow

    let mut z: usize = 0;
    let tmp: &'t mut usize = &mut z;
    let _h: fn() = to_fn(move || { set::<'t>(tmp); to_fn(move || z = 42); }); //~ ERROR cannot assign
}
