// Tests that two closures cannot simultaneously have mutable
// and immutable access to the variable. Issue #6801.

#![feature(box_syntax)]

fn get<'a>(x: &'a isize) -> isize {
    *x
}

fn set<'a>(x: &'a mut isize) {
    *x = 4;
}

fn c() {
    let mut x: isize = 3;
    let tmp0: &'a mut isize = &mut x;
    let c1: fn() -> () = || set::<'a>(tmp0);
    let tmp1: &'b isize = &x;
    let c2: fn() -> isize = || (*tmp1) * 5;
    //~^ ERROR cannot borrow `x` as immutable because it is also borrowed as mutable
    drop(c1);
}

fn main() {
}
