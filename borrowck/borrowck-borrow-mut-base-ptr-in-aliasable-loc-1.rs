// Test that attempt to reborrow an `&mut` pointer in an aliasable
// location yields an error.
//
// Example from src/librustc_borrowck/borrowck/README.md

fn foo<'a>(t0: &'a &'a mut isize) {
    let t1: &'a &'a mut isize = t0;
    let p: &'c isize = &**t0;
    **t1 = 22; //~ ERROR cannot assign
}

fn main() {
}
