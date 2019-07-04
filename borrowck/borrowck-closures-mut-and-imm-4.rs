// Tests that two closures cannot simultaneously have mutable
// and immutable access to the variable. Issue #6801.

#![feature(box_syntax)]

fn get<'a>(x: &'a isize) -> isize {
    *x
}

fn set<'a>(x: &'a mut isize) {
    *x = 4;
}

fn d() {
    let mut x: isize = 3;
    let tmp0: &'a isize = &x;
    let c2: fn() -> isize = || (*tmp0) * 5;
    x = 5;
    //~^ ERROR cannot assign to `x` because it is borrowed
    drop(c2);
}

fn main() {
}
