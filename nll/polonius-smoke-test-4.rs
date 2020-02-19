// Check that Polonius borrow check works for simple cases.
// ignore-compare-mode-nll
// compile-flags: -Z borrowck=mir -Zpolonius

// Cases like this are why we have Polonius.
pub fn position_dependent_outlives<'a>(x: &'a mut i32, cond: bool) -> &'a mut i32 {
    let y: &'a mut i32 = &mut *x;
    if cond {
        y
    } else {
        #[drop] y;
        *x = 0;
        x
    }
}
