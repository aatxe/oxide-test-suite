// Unit test for the "user substitutions" that are annotated on each
// node.

struct SomeStruct<T>(T);

fn annot_reference_named_lifetime_in_closure<'a>(_: &'a u32) {
    let _closure = || {
        let c = 66;
        SomeStruct::<&'a u32>(&c); //~ ERROR
    };
}
