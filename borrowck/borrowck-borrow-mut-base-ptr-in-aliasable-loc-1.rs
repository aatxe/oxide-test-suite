// Test that attempt to reborrow an `&mut` pointer in an aliasable
// location yields an error.
//
// Example from src/librustc_borrowck/borrowck/README.md

fn foo<'a, 'b>(t0: &'a &'b mut isize) {
    let t1: &'a &'b mut isize = t0;
    let p: &'c isize = &**t0;
    **t1 = 22; //~ ERROR cannot assign
}

fn main() {
}
