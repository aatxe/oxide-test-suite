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

fn d<'a>(x: &'a mut isize) {
    let tmp0 = #[lft="t0"] &mut *x;
    let c1 = || set::<'t0>(tmp0);
    let tmp1 = #[lft="t1"] &mut *x;
    let c2 = || set::<'t1>(tmp1); //~ ERROR two closures require unique access to `x` at the same time
    c1;
}

fn main() {
}
