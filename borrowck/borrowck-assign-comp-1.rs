struct Point { x: isize, y: isize }

fn a() {
    let mut p: Point = Point {x: 3, y: 4};
    let q: &'q Point = &p;

    // This assignment is illegal because the field x is not
    // inherently mutable; since `p` was made immutable, `p.x` is now
    // immutable.  Otherwise the type of &_q.x (&isize) would be wrong.
    p.x = 5; //~ ERROR cannot assign to `p.x` because it is borrowed
    q.x;
}

fn main() {
}
