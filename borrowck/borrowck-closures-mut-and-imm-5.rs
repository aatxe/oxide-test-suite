// Tests that two closures cannot simultaneously have mutable
// and immutable access to the variable. Issue #6801.

#![feature(box_syntax)]

fn get<'a>(x: &'a isize) -> isize {
    *x
}

fn set<'a>(x: &'a mut isize) {
    *x = 4;
}

fn e() {
    let mut x: isize = 3;
    let tmp0: &'a isize = &x;
    let c1: fn() -> isize = || get::<'a>(tmp0);
    x = 5;
    //~^ ERROR cannot assign to `x` because it is borrowed
    drop::<fn() -> isize>(c1);
}

fn main() {
}
