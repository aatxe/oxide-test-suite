// check that existing borrows due to a closure capture give a special note

fn borrow_mut_while_borrowed_mut(mut x: i32) {
    let tmp0: &'t0 mut i32 = &mut x;
    let f: fn() = || *tmp0 = 0;
    let y: &'y mut i32 = &mut x; //~ ERROR
    let tmp1: &'t1 fn() = &f;
    use_ref::<'t1, fn()>(tmp1);
}

fn main() {}

fn use_mut<'a, T>(x: &'a mut T) { } fn use_ref<'a, T>(x: &'a T) { }
