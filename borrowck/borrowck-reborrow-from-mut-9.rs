struct Foo {
  bar1: Bar,
  bar2: Bar
}

struct Bar {
  int1: isize,
  int2: isize,
}

fn borrow_mut_and_base_mut<'a>(foo: &'a mut Foo) {
    let _bar1: &'b1 mut isize = &mut (*foo).bar1.int1;
    let _foo1: &'f1 mut Bar = &mut (*foo).bar1; //~ ERROR cannot borrow
    use_mut::<'b1, isize>(_bar1);
}
fn main() {}

fn use_mut<'a, T>(_: &'a mut T) { }
fn use_imm<'a, T>(_: &'a T) { }
