// Tests that two closures cannot simultaneously both have mutable
// access to the variable. Related to issue #6801.

fn get<'a>(x: &'a isize) -> isize {
    *x
}

fn set<'a>(x: &'a mut isize) {
    *x = 4;
}

fn a<'a>(x: &'a mut isize) {
    let tmp0: &'b mut isize = &mut *x;
    let mut c1: fn() -> () = || set::<'b>(tmp0);
    let tmp1: &'c mut isize = &mut *x;
    let mut c2: fn() -> () = || set::<'c>(tmp1);
    //~^ ERROR two closures require unique access to `x` at the same time
    c2(); c1();
}

fn main() {
}
