// Unit test for the "user substitutions" that are annotated on each
// node.

struct SomeStruct<T>(T);

fn no_annot() {
    let c = 66;
    SomeStruct::<&'c u32>(#[lft = "c"] &c);
}
