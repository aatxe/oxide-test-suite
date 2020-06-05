// Check that Polonius borrow check works for simple cases.
// ignore-compare-mode-nll
// compile-flags: -Z borrowck=mir -Zpolonius

fn foo<'a, 'b>(p: &'b &'a mut usize) -> &'b usize where 'a: 'b {
    #[lft = "c"] &**p
}

// Check that we create constraints for well-formedness of function arguments
fn well_formed_function_inputs() {
    let tmp0: u32 = 1;
    let s: &'s mut u32 = &mut tmp0;
    let r: &'r mut u32 = &mut *s;
    let tmp1: &'t1 &'r mut u32 = &r;
    let tmp: &'t1 u32 = foo::<'r, 't1>(tmp1);
    s; //~ ERROR
    tmp;
}

fn main() {}
