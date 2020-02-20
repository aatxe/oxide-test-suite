// Unit test for the "user substitutions" that are annotated on each
// node.

fn some_fn<T>(arg: T) { }

fn annot_underscore() {
    let c = 66;
    some_fn::<&'c u32>(#[lft = "c"] &c); // OK
}
