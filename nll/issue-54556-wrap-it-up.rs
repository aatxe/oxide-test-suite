// This is testing how the diagnostic from issue #54556 behaves when
// the destructor code is attached to a place held in a field of the
// temporary being dropped.
//
// Eventually it would be nice if the diagnostic would actually report
// that specific place and its type that implements the `Drop` trait.
// But for the short term, it is acceptable to just print out the
// whole type of the temporary.

#![allow(warnings)]

struct Wrap<'p> { p: &'p mut i32 }

fn drop_wrapper<'a, 'p>(wrapper: &'a mut Wrap<'p>) where 'p: 'a {
    *((*wrapper).p) += 1;
}

// dummy string
struct String;

struct Foo<'p> { a: String, b: Wrap<'p> }

fn main() {
    let mut x: i32 = 0;
    let tmp0: &'t0 mut i32 = &mut x;
    let wrap: Wrap<'t0> = Wrap::<'t0> { p: tmp0 };
    let s: String = String();
    let foo: Foo<'t0> = Foo::<'t0> { a: s, b: wrap };
    x = 1; //~ ERROR cannot assign to `x` because it is borrowed [E0506]
    let tmp1: &'t1 mut Wrap<'t0> = &mut foo.b;
    drop_wrapper::<'t1, 't0>(tmp1);
}
