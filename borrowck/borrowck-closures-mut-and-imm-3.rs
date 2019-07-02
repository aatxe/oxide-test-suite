// Tests that two closures cannot simultaneously have mutable
// and immutable access to the variable. Issue #6801.

#![feature(box_syntax)]

fn get(x: &isize) -> isize {
    *x
}

fn set(x: &mut isize) {
    *x = 4;
}

fn c() {
    let mut x = 3;
    let c1 = || set(&mut x);
    let c2 = || x * 5;
    //~^ ERROR cannot borrow `x` as immutable because it is also borrowed as mutable
    drop(c1);
}

fn main() {
}
