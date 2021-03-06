// Test that assignments to an `&mut` pointer which is found in a
// borrowed (but otherwise non-aliasable) location is illegal.

struct SS<'a> {
    pointer: &'a mut isize
}

fn a<'a>(s: &'a SS<'a>) {
    *(*s).pointer += 1; //~ ERROR cannot assign
}

fn main() {}
