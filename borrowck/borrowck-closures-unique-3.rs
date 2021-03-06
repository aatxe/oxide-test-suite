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

fn c<'a>(x: &'a mut isize) -> &'a mut isize {
    let tmp0 = #[lft="t0"] &*x;
    let c1 = || get::<'t0>(tmp0);
    let tmp1 = #[lft="t1"] &mut *x;
    let c2 = || {
        get::<'t1>(#[lft="t1"] &*tmp1);
        set::<'t1>(tmp1);
    }; //~ ERROR closure requires unique access to `x`
    let foo: &'a mut isize = tmp1;
    foo
}

fn main() {
}
