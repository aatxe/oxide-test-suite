// Tests that two closures cannot simultaneously have mutable
// and immutable access to the variable. Issue #6801.

#![feature(box_syntax)]

fn get<'a>(x: &'a isize) -> isize {
    *x
}

fn set<'a>(x: &'a mut isize) {
    *x = 4;
}

fn h() {
    struct Foo {
        f: Box<isize>
    }

    let mut x: Box<Foo> = box Foo { f: box 3 };
    let tmp0: &'a isize = &*x.f;
    let c1: fn() -> isize = || get::<'a>(tmp0);
    let tmp1: &'b mut isize = &mut *(*x).f
    let c2: fn() -> () = || *tmp1 = 5;
    //~^ ERROR cannot borrow `x` as mutable because it is also borrowed as immutable
    drop(c1);
}

fn main() {
}
