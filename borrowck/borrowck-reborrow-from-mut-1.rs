struct Foo {
  bar1: Bar,
  bar2: Bar
}

struct Bar {
  int1: isize,
  int2: isize,
}

fn borrow_same_field_twice_mut_mut<'a>(foo: &'a mut Foo) {
    let _bar1: &'b1 mut Bar = &mut foo.bar1;
    let _bar2: &'b2 mut Bar = &mut foo.bar1;  //~ ERROR cannot borrow
    use_mut(_bar1);
}
fn main() {}

fn use_mut<'a, T>(_: &'a mut T) { }
fn use_imm<'a, T>(_: &'a T) { }
