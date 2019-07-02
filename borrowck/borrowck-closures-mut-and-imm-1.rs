// Tests that two closures cannot simultaneously have mutable
// and immutable access to the variable. Issue #6801.

#![feature(box_syntax)]

fn get(x: &isize) -> isize {
    *x
}

fn set(x: &mut isize) {
    *x = 4;
}

fn a() {
    let mut x = 3;
    let c1 = || x = 4;
    let c2 = || x * 5;
    //~^ ERROR cannot borrow `x` as immutable because it is also borrowed as mutable
    drop(c1);
}

fn main() {
}
