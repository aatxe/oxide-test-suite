// Unit test for the "user substitutions" that are annotated on each
// node.

struct SomeStruct<T>(T);

fn annot_reference_any_lifetime() {
    let c = 66;
    SomeStruct::<&'c u32>(#[lft = "c"] &c);
}
