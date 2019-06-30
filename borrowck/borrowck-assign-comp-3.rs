struct Point { x: isize, y: isize }

fn d() {
    // just for completeness's sake, the easy case, where we take the
    // address of a subcomponent and then modify that subcomponent:

    let mut p: Point = Point {x: 3, y: 4};
    let q: &'q isize = &p.y;
    p.y = 5; //~ ERROR cannot assign to `p.y` because it is borrowed
    *q;
}

fn main() {
}
