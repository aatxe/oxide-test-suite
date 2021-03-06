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

fn borrow_both_mut_pattern() {
    let mut foo: Box<Foo> = make_foo();
    match *foo {
        Foo { bar1: ref mut _bar1, bar2: ref mut _bar2 } => {
            *_bar1;
            *_bar2;
        }
    }
}

fn main() {}
