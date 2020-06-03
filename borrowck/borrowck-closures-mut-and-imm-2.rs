// Tests that two closures cannot simultaneously have mutable
// and immutable access to the variable. Issue #6801.

#![feature(box_syntax)]

fn get<'a>(x: &'a isize) -> isize {
    *x
}

fn set<'a>(x: &'a mut isize) {
    *x = 4;
}

fn b() {
    let mut x = 3;
    let tmp0 = #[lft="t0"] &mut x;
    let c1 = || set::<'t0>(tmp0);
    let tmp1 = #[lft="t1"] &x;
    let c2 = || get::<'t1>(tmp1);
    //~^ ERROR cannot borrow `x` as immutable because it is also borrowed as mutable
    drop::<fn() -> ()>(c1);
}

fn main() {
}
