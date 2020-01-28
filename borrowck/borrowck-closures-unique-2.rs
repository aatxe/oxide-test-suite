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

fn b<'a>(x: &'a mut isize) {
    let c1: fn() -> isize = || get::<'a>(x);
    let c2: fn() -> isize = || set::<'a>(x); //~ ERROR closure requires unique access to `x`
    c1;
}

fn main() {
}
