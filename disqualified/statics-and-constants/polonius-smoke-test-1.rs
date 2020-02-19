// Check that Polonius borrow check works for simple cases.
// ignore-compare-mode-nll
// compile-flags: -Z borrowck=mir -Zpolonius

pub fn return_ref_to_local() -> &'static i32 {
    let x = 0;
    &x //~ ERROR
}
