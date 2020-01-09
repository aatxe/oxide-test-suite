#![feature(box_syntax)]



// struct Foo(Box<isize>, isize);

struct Bar(isize, isize);

fn main() {
    let mut x: Bar = Bar(1, 2);
    let a: &'a mut isize = &mut x.0;
    let b: &'b mut isize = &mut x.0; //~ ERROR cannot borrow `x.0` as mutable more than once at a time
    use_mut(a);
}

fn use_mut<'a, T>(x: &'a mut T) { } fn use_ref<'a, T>(x: &'a T) { }
