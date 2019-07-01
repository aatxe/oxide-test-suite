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

fn borrow_same_field_twice_imm_mut() {
    let mut foo: Foo = make_foo();
    let bar1: &'a Bar = &foo.bar1;
    let _bar2: &'b mut Bar = &mut foo.bar1;  //~ ERROR cannot borrow
    *bar1;
}

fn main() {}
