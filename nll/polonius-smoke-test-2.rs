// Check that Polonius borrow check works for simple cases.
// ignore-compare-mode-nll
// compile-flags: -Z borrowck=mir -Zpolonius

pub fn use_while_mut() {
    let mut x: u32 = 0;
    let y: &'y mut u32 = &mut x;
    let z: u32 = x; //~ ERROR
    let w: &'y mut u32 = y;
}
