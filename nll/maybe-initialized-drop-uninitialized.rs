// compile-flags: -Zborrowck=mir
// compile-pass

#![allow(warnings)]

struct Wrap<'p> { p: &'p mut i32 }

fn drop_wrapper<'a, 'b>(wrap: &'a mut Wrap<'b>) where 'b: 'a {
    *(*wrap).p += 1;
}

fn main() {
    let mut x: i32 = 0;
    let tmp0: &'t0 mut i32 = &mut x;
    let wrap: Wrap<'t0> = Wrap::<'t0> { p: tmp0  };
    let tmp1: &'t1 mut Wrap<'t0> = &mut wrap;
    drop_wrapper::<'t1, 't0>(tmp1);
    #[drop] wrap;
    x = 1; // OK, drop is inert
}
