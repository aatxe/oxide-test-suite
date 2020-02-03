struct Point(u32, u32);

fn main() {
    let pt: Point = Point(6, 9);
    let x: &'x mut Point = &mut pt;
    drop::<&'x mut Point>(x);
    let y: &'y mut Point = &mut pt;
    let _val: Point = *x; // Should get rejected, x's lifetime has ended.
}
