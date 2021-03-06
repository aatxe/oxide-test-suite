#[derive(Copy, Clone)]
struct Foo {
  bar1: Bar,
  bar2: Bar
}

#[derive(Copy, Clone)]
struct Bar {
  int1: isize,
  int2: isize,
}

fn make_foo() -> Box<Foo> { panic!() }

fn borrow_mut_and_imm() {
    let mut foo: Box<Foo> = make_foo();
    let bar1: &'a mut Bar = &mut foo.bar1;
    let _foo1: &'b Bar = &foo.bar2;
}

fn main() {}
