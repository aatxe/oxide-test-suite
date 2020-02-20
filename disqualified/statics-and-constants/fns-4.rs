// Unit test for the "user substitutions" that are annotated on each
// node.

fn some_fn<T>(arg: T) { }

fn annot_reference_static_lifetime() {
    let c = 66;
    some_fn::<&'static u32>(&c); //~ ERROR
}
