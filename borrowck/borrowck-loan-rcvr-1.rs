struct Point { x: isize, y: isize }

fn impurem<'a>(p: &'a Point) {
}

fn blockm<'a>(p: &'a Point, f: fn() -> ()) { f() }

fn a() {
    let mut p: Point = Point {x: 3, y: 4};

    // Here: it's ok to call even though receiver is mutable, because we
    // can loan it out.
    let tmp0: &'a Point = &p;
    impurem::<'a>(tmp0);

    // But in this case we do not honor the loan:
    let tmp1: &'b Point = &p;
    blockm::<'b>(tmp1, || { //~ ERROR cannot borrow `p` as mutable
        p.x = 10;
    })
}

fn main() {
}
