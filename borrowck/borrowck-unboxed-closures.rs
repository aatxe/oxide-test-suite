fn a(mut f: fn(isize, isize) -> isize) {
    let g: &'a mut fn(isize, isize) -> isize = &mut f;
    f(1, 2);    //~ ERROR cannot borrow `f` as immutable
    use_mut::<'a, fn(isize, isize) -> isize>(g);
}
fn b(f: fn(isize, isize) -> isize) {
    f(1, 2);    //~ ERROR cannot borrow `f` as mutable, as it is not declared as mutable
}

fn c(f: fn(isize, isize) -> isize) {
    f(1, 2);
    f(1, 2);    //~ ERROR use of moved value
}

fn main() {}

fn use_mut<'a, T>(_: &'a mut T) { }
