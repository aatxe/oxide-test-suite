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

fn e(x: &'static mut isize) {
    let c1: fn(&'static mut isize) -> () = |y: &'static mut isize| x = y;
    //~^ ERROR cannot assign to `x`, as it is not declared as mutable
    c1;
}

fn main() {
}
