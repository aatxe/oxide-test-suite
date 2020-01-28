struct Foo {
  bar1: Bar,
  bar2: Bar
}

struct Bar {
  int1: isize,
  int2: isize,
}

fn borrow_mut_and_base_imm<'a>(foo: &'a mut Foo) {
    let _bar1: &'b1 mut isize = &mut (*foo).bar1.int1;
    let _foo1: &'f1 Bar = &(*foo).bar1; //~ ERROR cannot borrow
    let _foo2: &'f2 Foo = &*foo; //~ ERROR cannot borrow
    use_mut::<'b1, isize>(_bar1);
}
fn main() {}

fn use_mut<'a, T>(_: &'a mut T) { }
fn use_imm<'a, T>(_: &'a T) { }
