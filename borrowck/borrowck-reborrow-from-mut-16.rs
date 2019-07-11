struct Foo {
  bar1: Bar,
  bar2: Bar
}

struct Bar {
  int1: isize,
  int2: isize,
}

fn borrow_long_path_both_mut<'a>(foo: &'a mut Foo) {
    let _bar1: &'b1 mut isize = &mut foo.bar1.int1;
    let _foo1: &'f1 mut isize = &mut foo.bar2.int2;
    use_mut(_bar1);
}
fn main() {}

fn use_mut<'a, T>(_: &'a mut T) { }
fn use_imm<'a, T>(_: &'a T) { }
