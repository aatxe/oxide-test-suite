// Tests that two closures cannot simultaneously have mutable
// and immutable access to the variable. Issue #6801.

#![feature(box_syntax)]

fn get(x: &isize) -> isize {
    *x
}

fn set(x: &mut isize) {
    *x = 4;
}

fn h() {
    struct Foo {
        f: Box<isize>
    }

    let mut x: Box<_> = box Foo { f: box 3 };
    let c1 = || get(&*x.f);
    let c2 = || *x.f = 5;
    //~^ ERROR cannot borrow `x` as mutable because it is also borrowed as immutable
    drop(c1);
}

fn main() {
}
