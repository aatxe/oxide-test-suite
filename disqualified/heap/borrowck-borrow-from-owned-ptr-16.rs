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

fn borrow_long_path_both_mut() {
    let mut foo: Box<Foo> = make_foo();
    let bar1: &'a mut isize = &mut foo.bar1.int1;
    let foo1: &'b mut isize = &mut foo.bar2.int2;
    *bar1;
    *foo1;
}

fn main() {}
