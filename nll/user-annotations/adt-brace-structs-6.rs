// Unit test for the "user substitutions" that are annotated on each
// node.

struct SomeStruct<T> { t: T }

fn annot_reference_named_lifetime_ok<'a>(c: &'a u32) {
    SomeStruct::<&'a u32> { t: c };
}
