struct Wrap<'p> { p: &'p mut i32 }

fn drop_wrapper<'a, 'b>(wrap: &'a mut Wrap<'b>) where 'b: 'a {
    *(*wrap).p += 1;
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
    let tmp1: &'t1 mut Wrap<'t0> = &mut foo.b;
    drop_wrapper::<'t1, 't0>(tmp1);
    #[drop] foo.b;
    x = 1; //~ ERROR cannot assign to `x` because it is borrowed [E0506]
    // FIXME ^ Should not error in the future with implicit dtors, only manually implemented ones
}
