// Tests that two closures cannot simultaneously have mutable
// access to the variable, whether that mutable access be used
// for direct assignment or for taking mutable ref. Issue #6801.

fn to_fn_mut(f: fn() -> ()) -> fn() -> () { f }

fn set<'a>(x: &'a mut isize) {
    *x = 4;
}

fn d() {
    let mut x: usize = 3;
    let tmp0: &'t0 mut usize = &mut x;
    let c1: fn() -> () = to_fn_mut(|| *tmp0 = 5);
    let tmp1: &'t1 mut usize = &mut x;
    let c2: fn() -> () = to_fn_mut(|| {
        let _y: fn() -> () = to_fn_mut(|| set::<'t1>(tmp1));
    }); // (nested closure)
    //~^ ERROR cannot borrow `x` as mutable more than once
    drop::<(fn() -> (), fn() -> ())>((c1, c2));
}

fn main() {
}
