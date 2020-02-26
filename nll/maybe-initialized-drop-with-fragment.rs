//compile-flags: -Zborrowck=mir

#![allow(warnings)]

struct Wrap<'p> { p: &'p mut i32 }

fn drop_wrap<'a, 'b>(wrap: &'a mut Wrap<'b>) where 'b: 'a {
    *(*wrap).p += 1;
}

// dummy string
struct String;

struct Foo<'p> { a: String, b: Wrap<'p> }

fn main() {
    let mut x = 0;
    let tmp0: &'t0 mut i32 = &mut x;
    let wrap = Wrap::<'t0> { p: tmp0 };
    let s = String();
    let foo = Foo::<'t0> { a: s, b: wrap };
    #[drop] foo.a;
    x = 1; //~ ERROR cannot assign to `x` because it is borrowed [E0506]
    let tmp1 = #[lft="t1"] &mut foo.b;
    drop_wrap::<'t1, 't0>(tmp1);
}
