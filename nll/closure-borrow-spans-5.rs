// check that existing borrows due to a closure capture give a special note

fn copy_while_borrowed_mut(mut x: i32) {
    let tmp0: &'t0 mut i32 = &mut x;
    let f: fn() = || *tmp0 = 0;
    let y: i32 = x; //~ ERROR
    let tmp1: &'t1 fn() = &f;
    use_ref::<'t1, fn()>(f);
}

fn main() {}

fn use_mut<'a, T>(x: &'a mut T) { } fn use_ref<'a, T>(x: &'a T) { }
