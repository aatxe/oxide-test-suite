struct Point { x: isize, y: isize }

fn c() {
    // this is sort of the opposite.  We take a loan to the interior of `p`
    // and then try to overwrite `p` as a whole.

    let mut p: Point = Point {x: 3, y: 4};
    let q: &'q isize = &p.y;
    p = Point {x: 5, y: 7};//~ ERROR cannot assign to `p` because it is borrowed
    p.x; // silence warning
    *q; // stretch loan
}

fn main() {
}
