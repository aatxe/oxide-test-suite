// Unit test for the "user substitutions" that are annotated on each
// node.

fn method<'a, T, U>(slf: &'a T, arg: T, arg2: U) { }

fn annot_reference_named_lifetime<'a>(_d: &'a u32) {
    let a = 22;
    let b = 44;
    let c = 66;
    method::<'t, u32, &'a u32>(#[lft="t"] &a, b, &c); //~ ERROR
}
