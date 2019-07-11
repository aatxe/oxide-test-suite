struct Foo {
  bar1: Bar,
  bar2: Bar
}

struct Bar {
  int1: isize,
  int2: isize,
}

fn borrow_same_field_twice_mut_mut<'a>(foo: &'a mut Foo) {
    let _bar1: &'b1 mut Bar = &mut foo.bar1;
    let _bar2: &'b2 mut Bar = &mut foo.bar1;  //~ ERROR cannot borrow
    use_mut(_bar1);
}
fn borrow_same_field_twice_mut_imm<'a>(foo: &'a mut Foo) {
    let _bar1: &'b1 mut Bar = &mut foo.bar1;
    let _bar2: &'b2 Bar = &foo.bar1;  //~ ERROR cannot borrow
    use_mut(_bar1);
}
fn borrow_same_field_twice_imm_mut<'a>(foo: &'a mut Foo) {
    let _bar1: &'b1 Bar = &foo.bar1;
    let _bar2: &'b2 Bar = &mut foo.bar1;  //~ ERROR cannot borrow
    use_imm(_bar1);
}
fn borrow_same_field_twice_imm_imm<'a>(foo: &'a mut Foo) {
    let _bar1: &'b1 Bar = &foo.bar1;
    let _bar2: &'b2 Bar = &foo.bar1;
    use_imm(_bar1);
}
fn borrow_both_mut<'a>(foo: &'a mut Foo) {
    let _bar1: &'b1 mut Bar = &mut foo.bar1;
    let _bar2: &'b2 mut Bar = &mut foo.bar2;
    use_mut(_bar1);
}
fn borrow_both_mut_pattern<'a>(foo: &'a mut Foo) {
    match *foo {
        Foo { bar1: ref mut _bar1, bar2: ref mut _bar2 } =>
        { use_mut(_bar1); use_mut(_bar2); }
    }
}
fn borrow_var_and_pattern<'a>(foo: &'a mut Foo) {
    let _bar1: &'b1 mut Bar = &mut foo.bar1;
    match *foo {
        Foo { bar1: ref mut _bar1, bar2: _ } => {}
        //~^ ERROR cannot borrow
    }
    use_mut(_bar1);
}
fn borrow_mut_and_base_imm<'a>(foo: &'a mut Foo) {
    let _bar1: &'b1 mut isize = &mut foo.bar1.int1;
    let _foo1: &'f1 Bar = &foo.bar1; //~ ERROR cannot borrow
    let _foo2: &'f2 Foo = &*foo; //~ ERROR cannot borrow
    use_mut(_bar1);
}
fn borrow_mut_and_base_mut<'a>(foo: &'a mut Foo) {
    let _bar1: &'b1 mut isize = &mut foo.bar1.int1;
    let _foo1: &'f1 mut Bar = &mut foo.bar1; //~ ERROR cannot borrow
    use_mut(_bar1);
}
fn borrow_mut_and_base_mut2<'a>(foo: &'a mut Foo) {
    let _bar1: &'b1 mut isize = &mut foo.bar1.int1;
    let _foo2: &'f1 mut Foo = &mut *foo; //~ ERROR cannot borrow
    use_mut(_bar1);
}
fn borrow_imm_and_base_mut<'a>(foo: &'a mut Foo) {
    let _bar1: &'b1 isize = &foo.bar1.int1;
    let _foo1: &'f1 mut Bar = &mut foo.bar1; //~ ERROR cannot borrow
    use_imm(_bar1);
}
fn borrow_imm_and_base_mut2<'a>(foo: &'a mut Foo) {
    let _bar1: &'b1 isize = &foo.bar1.int1;
    let _foo2: &'f1 mut Foo = &mut *foo; //~ ERROR cannot borrow
    use_imm(_bar1);
}
fn borrow_imm_and_base_imm<'a>(foo: &'a mut Foo) {
    let _bar1: &'b1 isize = &foo.bar1.int1;
    let _foo1: &'f1 Bar = &foo.bar1;
    let _foo2: &'f2 Foo = &*foo;
    use_imm(_bar1);
}
fn borrow_mut_and_imm<'a>(foo: &'a mut Foo) {
    let _bar1: &'b1 mut Bar = &mut foo.bar1;
    let _foo1: &'b2 Bar = &foo.bar2;
    use_mut(_bar1);
}
fn borrow_mut_from_imm<'a>(foo: &'a Foo) {
    let _bar1: &'mut Bar = &mut foo.bar1; //~ ERROR cannot borrow
}

fn borrow_long_path_both_mut<'a>(foo: &'a mut Foo) {
    let _bar1: &'b1 mut isize = &mut foo.bar1.int1;
    let _foo1: &'f1 mut isize = &mut foo.bar2.int2;
    use_mut(_bar1);
}
fn main() {}

fn use_mut<'a, T>(_: &'a mut T) { }
fn use_imm<'a, T>(_: &'a T) { }
