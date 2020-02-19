//compile-flags: -Zborrowck=mir

#![allow(warnings)]

struct Wrap<'p> { p: &'p mut i32 }

fn drop_wrap<'a, 'b>(wrap: &'a mut Wrap<'b>) where 'b: 'a {
    *(*wrap).p += 1;
}

fn main() {
    let mut x = 0;
    let tmp0: &'t0 mut i32 = &mut x;
    let wrap = Wrap::<'t0> { p: tmp0 };
    x = 1; //~ ERROR cannot assign to `x` because it is borrowed [E0506]
    let tmp1: &'t1 mut Wrap<'t0> = &mut wrap;
    drop_wrap::<'t1, 't0>(tmp1);
}
