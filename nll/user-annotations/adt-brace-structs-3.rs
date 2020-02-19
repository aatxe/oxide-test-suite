// Unit test for the "user substitutions" that are annotated on each
// node.

struct SomeStruct<T> { t: T }

fn annot_reference_any_lifetime() {
    let c = 66;
    let tmp: &'t u32 = &c;
    SomeStruct::<&'t u32> { t: tmp };
}
