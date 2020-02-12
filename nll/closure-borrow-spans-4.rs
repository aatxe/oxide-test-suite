// check that existing borrows due to a closure capture give a special note

fn assign_while_borrowed(mut x: i32) {
    let tmp0: &'t0 i32 = &x;
    let f: fn() -> i32 = || *tmp0;
    x = 1; //~ ERROR
    let tmp1: &'t1 fn() -> i32 = &f;
    use_ref::<'t1, fn() -> i32>(tmp1);
}

fn main() {}

fn use_mut<'a, T>(x: &'a mut T) { } fn use_ref<'a, T>(x: &'a T) { }
