// Unit test for the "user substitutions" that are annotated on each
// node.

struct SomeStruct<T>(T);

fn annot_reference_static_lifetime() {
    let c = 66;
    SomeStruct::<&'static u32>(&c); //~ ERROR
}
