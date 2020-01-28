// Tests that two closures cannot simultaneously have mutable
// access to the variable, whether that mutable access be used
// for direct assignment or for taking mutable ref. Issue #6801.

fn to_fn_mut(f: fn() -> ()) -> fn() -> () { f }

fn set<'a>(x: &'a mut isize) {
    *x = 4;
}

struct Foo {
    f: Box<isize>
}

fn g() {
    let mut x: Box<Foo> = Box::new(Foo { f: Box::new(3) });
    let tmp0: &'t0 mut isize = &mut *(x.f);
    let c1: fn() -> () = to_fn_mut(|| set::<'t0>());
    let tmp1: &'t1 mut isize = &mut *(x.f);
    let c2: fn() -> () = to_fn_mut(|| set::<'t1>());
    //~^ ERROR cannot borrow `x` as mutable more than once
    drop::<(fn() -> (), fn() -> ())>((c1, c2));
}

fn main() {
}
