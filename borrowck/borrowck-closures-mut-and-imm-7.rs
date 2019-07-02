// Tests that two closures cannot simultaneously have mutable
// and immutable access to the variable. Issue #6801.

#![feature(box_syntax)]

fn get(x: &isize) -> isize {
    *x
}

fn set(x: &mut isize) {
    *x = 4;
}

fn g() {
    struct Foo {
        f: Box<isize>
    }

    let mut x: Box<_> = box Foo { f: box 3 };
    let c1 = || get(&*x.f);
    *x.f = 5;
    //~^ ERROR cannot assign to `*x.f` because it is borrowed
    drop(c1);
}

fn main() {
}
