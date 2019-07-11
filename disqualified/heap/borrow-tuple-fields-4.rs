#![feature(box_syntax)]



struct Foo(Box<isize>, isize);

struct Bar(isize, isize);

fn main() {
    let x: Foo = Foo(Box::new(1), 2);
    let r: &'r isize = &x.0;
    let y: Foo = x; //~ ERROR cannot move out of `x` because it is borrowed
    use_ref(r);
}

fn use_mut<'a, T>(x: &'a mut T) { } fn use_ref<'a, T>(x: &'a T) { }
