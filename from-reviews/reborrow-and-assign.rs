fn main() {
    let mut a: u32 = 0;
    let mut b: u32 = 0;
    let mut x: &'x mut u32 = &mut a;
    let y: &'y mut u32 = &mut *x; // y reborrows from x
    x = &mut b; // x gets reassigned
    // now both x and y can be used
    *y = 1;
    *x = 1;
}
