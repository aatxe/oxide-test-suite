// Unit test for the "user substitutions" that are annotated on each
// node.

fn method<'a, T, U>(slf: &'a T, arg: T, arg2: U) { }

fn no_annot() {
    let a = 22;
    let b = 44;
    let c = 66;
    method::<'a, u32, &'c u32>(#[lft = "a"] &a, b, #[lft="c"] &c); // OK
}
