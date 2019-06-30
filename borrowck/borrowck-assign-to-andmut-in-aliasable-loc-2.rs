// Test that assignments to an `&mut` pointer which is found in a
// borrowed (but otherwise non-aliasable) location is illegal.

struct S<'a> {
    pointer: &'a mut isize
}

fn b(s: &mut S) {
    *s.pointer += 1;
}

fn main() {}
