// Test that assignments to an `&mut` pointer which is found in a
// borrowed (but otherwise non-aliasable) location is illegal.

struct S<'a> {
    pointer: &'a mut isize
}

fn a(s: &S) {
    *s.pointer += 1; //~ ERROR cannot assign
}

fn main() {}
