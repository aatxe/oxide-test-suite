fn b(f: fn(isize, isize) -> isize) {
    f(1, 2);    //~ ERROR cannot borrow `f` as mutable, as it is not declared as mutable
}

fn main() {}

fn use_mut<'a, T>(_: &'a mut T) { }
