// Check that Polonius borrow check works for simple cases.
// ignore-compare-mode-nll
// compile-flags: -Z borrowck=mir -Zpolonius

fn foo<'a, 'b>(p: &'b &'a mut usize) -> &'b usize where 'a: 'b {
    &**p
}
