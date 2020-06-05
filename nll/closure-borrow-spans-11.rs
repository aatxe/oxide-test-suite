// check that existing borrows due to a closure capture give a special note

fn borrow_while_borrowed_unique<'a>(x: &'a mut i32) {
    let tmp0 = &mut x;
    let f = || **tmp0 = 0;
    let y = &*x; //~ ERROR
    let tmp1 = #[lft="t1"] &f;
    use_ref::<'t1, fn()>(tmp1);
}

fn main() {}

fn use_mut<'a, T>(x: &'a mut T) { } fn use_ref<'a, T>(x: &'a T) { }
