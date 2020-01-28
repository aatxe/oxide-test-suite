fn foo(x: Box<isize>) -> isize {
    let y: &'y isize = &*x;
    free(x); //~ ERROR cannot move out of `x` because it is borrowed
    *y
}

fn free(_x: Box<isize>) {
}

fn main() {
}
