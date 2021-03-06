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

fn borrow_mut_and_base_mut2() {
    let mut foo: Box<Foo> = make_foo();
    let bar1: &'a mut isize = &mut foo.bar1.int1;
    let _foo2: &mut Foo = &mut *foo; //~ ERROR cannot borrow
    *bar1;
}

fn main() {}
