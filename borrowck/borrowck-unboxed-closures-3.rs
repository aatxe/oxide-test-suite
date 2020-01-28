fn c(f: fn(isize, isize) -> isize) {
    f(1, 2);
    f(1, 2);    //~ ERROR use of moved value
}

fn main() {}

fn use_mut<'a, T>(_: &'a mut T) { }
