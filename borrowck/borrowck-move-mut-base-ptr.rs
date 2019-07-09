// Test that attempt to move `&mut` pointer while pointee is borrowed
// yields an error.
//
// Example from src/librustc_borrowck/borrowck/README.md



fn foo<'x>(t0: &'x mut isize) {
    let p: &'r isize = &*t0; // Freezes `*t0`
    let t1: &'x mut isize = t0;        //~ ERROR cannot move out of `t0`
    *t1 = 22;
    use_ref::<'r, isize>(p);
}

fn main() {
}

fn use_mut<'a, T>(x: &'a mut T) { } fn use_ref<'a, T>(x: &'a T) { }
