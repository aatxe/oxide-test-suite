fn a<F:Fn(isize, isize) -> isize>(mut f: F) {
    let g: &'a mut fn(isize, isize) -> isize = &mut f;
    f(1, 2);    //~ ERROR cannot borrow `f` as immutable
    use_mut::<'a, fn(isize, isize) -> isize>(g);
}
fn b<F:FnMut(isize, isize) -> isize>(f: F) {
    f(1, 2);    //~ ERROR cannot borrow `f` as mutable, as it is not declared as mutable
}

fn c<F:FnOnce(isize, isize) -> isize>(f: F) {
    f(1, 2);
    f(1, 2);    //~ ERROR use of moved value
}

fn main() {}

fn use_mut<'a, T>(_: &'a mut T) { }
