// Unit test for the "user substitutions" that are annotated on each
// node.

fn some_fn<T>(arg: T) { }

fn annot_reference_named_lifetime<'a>(_d: &'a u32) {
    let c = 66;
    some_fn::<&'a u32>(&c); //~ ERROR
}
