// Unit test for the "user substitutions" that are annotated on each
// node.

struct SomeStruct<T> { t: T }

fn annot_reference_named_lifetime<'a>(_d: &'a u32) {
    let c = 66;
    SomeStruct::<&'a u32> { t: &c }; //~ ERROR
}
