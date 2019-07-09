// Tests that two closures cannot simultaneously have mutable
// and immutable access to the variable. Issue #6801.

fn get<'a>(x: &'a isize) -> isize {
    *x
}

fn set<'a>(x: &'a mut isize) {
    *x = 4;
}

fn a<'a>(x: &'a isize) {
    let tmp0: &'b mut isize = &mut *x;
    let mut c1 = || set::<'b>(tmp0);
    //~^ ERROR cannot borrow
    let tmp1: &'c mut isize = &mut *x;
    let mut c2 = || set::<'c>(tmp1);
    //~^ ERROR cannot borrow
    //~| ERROR two closures require unique access to `x` at the same time
    c2(); c1();
}

fn main() {
}
