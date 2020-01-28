struct Foo {
  bar1: Bar,
  bar2: Bar
}

struct Bar {
  int1: isize,
  int2: isize,
}

fn borrow_mut_from_imm<'a>(foo: &'a Foo) {
    let _bar1: &'mut Bar = &mut (*foo).bar1; //~ ERROR cannot borrow
}

fn main() {}

fn use_mut<'a, T>(_: &'a mut T) { }
fn use_imm<'a, T>(_: &'a T) { }
