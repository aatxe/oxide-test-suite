// Tests that two closures cannot simultaneously have mutable
// access to the variable, whether that mutable access be used
// for direct assignment or for taking mutable ref. Issue #6801.

fn to_fn_mut(f: fn()) -> fn() { f }

fn a() {
    let mut x: usize = 3;
    let c1: fn() = to_fn_mut(|| x = 4);
    let c2: fn() = to_fn_mut(|| x = 5); //~ ERROR cannot borrow `x` as mutable more than once
    c1;
}

fn set<'a>(x: &'a mut isize) {
    *x = 4;
}

fn b() {
    let mut x: usize = 3;
    let tmp0: &'t0 mut usize = &mut x;
    let c1: fn() = to_fn_mut(|| set::<'t0>(tmp0));
    let tmp1: &'t1 mut usize = &mut x;
    let c2: fn() = to_fn_mut(|| set::<'t1>(tmp1)); //~ ERROR cannot borrow `x` as mutable more than once
    c1;
}

fn c() {
    let mut x: usize = 3;
    let c1: fn() = to_fn_mut(|| x = 5);
    let tmp0: &'t0 mut usize = &mut x;
    let c2: fn() = to_fn_mut(|| set::<'t0>(tmp0)); //~ ERROR cannot borrow `x` as mutable more than once
    c1;
}

fn d() {
    let mut x: usize = 3;
    let c1: fn() = to_fn_mut(|| x = 5);
    let tmp0: &'t0 mut usize = &mut x;
    let c2: fn() = to_fn_mut(|| {
        let _y: fn() = to_fn_mut(|| set::<'t0>(tmp0));
    }); // (nested closure)
    //~^ ERROR cannot borrow `x` as mutable more than once
    c1;
}

struct Foo {
    f: Box<isize>
}

fn g() {
    let mut x: Box<Foo> = Box::new(Foo { f: Box::new(3)});
    let tmp0: &'t0 mut isize = &mut (*x.f);
    let c1: fn() = to_fn_mut(|| set::<'t0>(tmp0));
    let tmp1: &'t1 mut isize = &mut (*x.f);
    let c2: fn() = to_fn_mut(|| set::<'t1>(tmp1));
    //~^ ERROR cannot borrow `x` as mutable more than once
    c1;
}

fn main() {
}
