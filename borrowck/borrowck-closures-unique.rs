// Tests that a closure which requires mutable access to the referent
// of an `&mut` requires a "unique" borrow -- that is, the variable to
// be borrowed (here, `x`) will not be borrowed *mutably*, but
//  may be *immutable*, but we cannot allow
// multiple borrows.



fn get<'a>(x: &'a isize) -> isize {
    *x
}

fn set<'a>(x: &'a mut isize) -> isize {
    *x
}

fn a<'a>(x: &'a mut isize) {
    let c1: fn() -> isize = || get(x);
    let c2: fn() -> isize = || get(x);
    c1();
    c2();
}

fn b<'a>(x: &'a mut isize) {
    let c1: fn() -> isize = || get(x);
    let c2: fn() -> isize = || set(x); //~ ERROR closure requires unique access to `x`
    c1;
}

fn c<'a>(x: &'a mut isize) {
    let c1: fn() -> isize = || get(x);
    let c2: fn() -> isize = || { get(x); set(x); }; //~ ERROR closure requires unique access to `x`
    c1;
}

fn d<'a>(x: &'a mut isize) {
    let c1: fn() -> isize = || set(x);
    let c2: fn() -> isize = || set(x); //~ ERROR two closures require unique access to `x` at the same time
    c1;
}

fn e(x: &'static mut isize) {
    let c1: fn(&'static mut isize) -> () = |y: &'static mut isize| x = y;
    //~^ ERROR cannot assign to `x`, as it is not declared as mutable
    c1;
}

fn f(x: &'static mut isize) {
    let c1: fn() -> () = || x = panic!(); // OK assignment is unreachable.
    c1;
}

fn main() {
}
