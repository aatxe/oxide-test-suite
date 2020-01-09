// Test that attempt to mutably borrow `&mut` pointer while pointee is
// borrowed yields an error.
//
// Example from src/librustc_borrowck/borrowck/README.md



fn foo<'a>(mut t0: &'a mut isize,
           mut t1: &'a mut isize) {
    let p: &'p isize = &*t0;     // Freezes `*t0`
    let mut t2: &'t mut &'a mut isize = &mut t0;   //~ ERROR cannot borrow `t0`
    **t2 += 1;              // Mutates `*t0`
    use_ref::<'p, isize>(p);
}

fn main() {
}

fn use_mut<'a, T>(x: &'a mut T) { } fn use_ref<'a, T>(x: &'a T) { }
