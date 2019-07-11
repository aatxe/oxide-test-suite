struct Foo {
  bar1: Bar,
  bar2: Bar
}

struct Bar {
  int1: isize,
  int2: isize,
}

fn borrow_both_mut_pattern<'a>(foo: &'a mut Foo) {
    match *foo {
        Foo { bar1: ref mut _bar1, bar2: ref mut _bar2 } =>
        { use_mut(_bar1); use_mut(_bar2); }
    }
}
fn main() {}

fn use_mut<'a, T>(_: &'a mut T) { }
fn use_imm<'a, T>(_: &'a T) { }
