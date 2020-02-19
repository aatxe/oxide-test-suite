// Basic test for liveness constraints: the region (`R1`) that appears
// in the type of `p` includes the points after `&v[0]` up to (but not
// including) the call to `use_x`. The `else` branch is not included.

#![allow(warnings)]
#![feature(rustc_attrs)]

struct MyStruct {
    field: bool
}

fn foo1() {
    let mut my_struct = MyStruct { field: false };

    let value = &my_struct.field;
    if *value {
        #[drop] value;
        my_struct.field = false;
    }
}

fn main() { }
