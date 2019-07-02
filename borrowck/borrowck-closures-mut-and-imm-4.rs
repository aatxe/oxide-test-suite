// Tests that two closures cannot simultaneously have mutable
// and immutable access to the variable. Issue #6801.

#![feature(box_syntax)]

fn get(x: &isize) -> isize {
    *x
}

fn set(x: &mut isize) {
    *x = 4;
}

fn d() {
    let mut x = 3;
    let c2 = || x * 5;
    x = 5;
    //~^ ERROR cannot assign to `x` because it is borrowed
    drop(c2);
}

fn main() {
}
