// Test that NLL analysis propagates lifetimes correctly through
// field accesses, Box accesses, etc.

#![feature(nll)]

fn foo<'a>(s: &'a mut (i32,)) -> i32 {
    let t: &'t mut (i32,) = &mut *s; // this borrow should last for the entire function
    let x: &'x i32  = &(*t).0;
    *s = (2,); //~ ERROR cannot assign to `*s`
    *x
}

fn main() {
    let tmp0: (i32,) = (0,);
    let tmp1: &'t1 mut (i32,) = &mut tmp0;
    foo::<'t1>(tmp1);
}
