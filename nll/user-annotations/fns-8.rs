// Unit test for the "user substitutions" that are annotated on each
// node.

fn some_fn<T>(arg: T) { }

fn annot_reference_named_lifetime_in_closure_ok<'a>(c: &'a u32) {
    let _closure = || {
        some_fn::<&'a u32>(c);
    };
}
