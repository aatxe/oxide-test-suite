fn a(mut f: fn(isize, isize) -> isize) {
    let g: &'a mut fn(isize, isize) -> isize = &mut f;
    f(1, 2);    //~ ERROR cannot borrow `f` as immutable
    use_mut::<'a, fn(isize, isize) -> isize>(g);
}

fn main() {}

fn use_mut<'a, T>(_: &'a mut T) { }
