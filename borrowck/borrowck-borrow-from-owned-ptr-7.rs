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

fn borrow_var_and_pattern() {
    let mut foo: Box<Foo> = make_foo();
    let bar1: &'a Bar = &mut foo.bar1;
    match *foo {
        Foo { bar1: ref mut _bar1, bar2: _ } => {}
        //~^ ERROR cannot borrow
    }
    *bar1;
}

fn main() {}
