// Tests that two closures cannot simultaneously have mutable
// and immutable access to the variable. Issue #6801.

#![feature(box_syntax)]

fn get(x: &isize) -> isize {
    *x
}

fn set(x: &mut isize) {
    *x = 4;
}

fn e() {
    let mut x = 3;
    let c1 = || get(&x);
    x = 5;
    //~^ ERROR cannot assign to `x` because it is borrowed
    drop(c1);
}

fn main() {
}
