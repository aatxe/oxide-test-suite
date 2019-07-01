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

fn make_foo() -> Foo { panic!() }

fn borrow_imm_and_base_imm() {
    let mut foo: Foo = make_foo();
    let bar1: &'a isize = &foo.bar1.int1;
    let _foo1: &'b Bar = &foo.bar1;
    let _foo2: &'c Foo = &foo;
    *bar1;
}

fn main() {}
