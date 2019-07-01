// Test that attempt to reborrow an `&mut` pointer in an aliasable
// location yields an error.
//
// Example from src/librustc_borrowck/borrowck/README.md

fn foo4<'a, 'b>(t0: &'a &'b mut isize) {
    let x: &'c mut isize = &mut **t0; //~ ERROR cannot borrow
    *x += 1;
}

fn main() {
}
