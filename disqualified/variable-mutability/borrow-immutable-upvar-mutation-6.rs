#![feature(unboxed_closures)]

// Tests that we can't assign to or mutably borrow upvars from `Fn`
// closures (issue #17780)

fn set<'a>(x: &'a mut usize) { *x = 5; }

fn to_fn<F: Fn()>(f: F) -> F { f }

fn main() {
    // By-value captures
    let mut z: usize = 0;
    let tmp: &'t mut usize = &mut z;
    let tmp0: fn() = move || {
        set::<'t>(tmp);
        let tmp1: fn() = move || z = 42; //~ ERROR cannot assign
        #[envs(tmp1)] to_fn(tmp1);
    };
    let _h: fn() = #[envs(tmp0)] to_fn(tmp0);
}
