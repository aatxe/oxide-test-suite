// Test that assignments to an `&mut` pointer which is found in a
// borrowed (but otherwise non-aliasable) location is illegal.

struct SS<'a> {
    pointer: &'a mut isize
}

fn b<'a>(s: &'a mut SS<'a>) {
    *(*s).pointer += 1;
}

fn main() {}
