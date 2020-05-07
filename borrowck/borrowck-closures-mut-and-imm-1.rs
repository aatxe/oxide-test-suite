// Tests that two closures cannot simultaneously have mutable
// and immutable access to the variable. Issue #6801.

#![feature(box_syntax)]

fn get<'a>(x: &'a isize) -> isize {
    *x
}

fn set<'a>(x: &'a mut isize) {
    *x = 4;
}

fn a() {
    let mut x: isize = 3;
    // Rust doesn't have writeable types for closures, so we use function types
    let c1: fn() -> () = {
        let tmp0: &'a mut isize = &mut x;
        || {
            *tmp0 = 4
        }
    };
    let c2: fn() -> isize = {
        let tmp0: &'b isize = &x;
        || {
            *tmp0 * 5
        }
    };
    //~^ ERROR cannot borrow `x` as immutable because it is also borrowed as mutable
    drop::<fn() -> ()>(c1);
}

fn main() {
}
