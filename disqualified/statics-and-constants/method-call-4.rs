// Unit test for the "user substitutions" that are annotated on each
// node.

fn method<'a, T, U>(slf: &'a T, arg: T, arg2: U) { }

fn annot_reference_static_lifetime() {
    let a = 22;
    let b = 44;
    let c = 66;
    method::<&'static u32>(&a, b,  &c); //~ ERROR
}
