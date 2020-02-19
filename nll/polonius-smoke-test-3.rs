// Check that Polonius borrow check works for simple cases.
// ignore-compare-mode-nll
// compile-flags: -Z borrowck=mir -Zpolonius

pub fn use_while_mut_fr<'a>(x: &'a mut i32) -> &'a mut i32 {
    let y: &'y mut i32 = &mut *x;
    let z: &'a mut i32 = x; //~ ERROR
    y
}
