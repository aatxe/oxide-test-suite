// Test that attempt to reborrow an `&mut` pointer in an aliasable
// location yields an error.
//
// Example from src/librustc_borrowck/borrowck/README.md

fn foo3<'a, 'b>(t0: &'a mut &'b mut isize) {
    let t1: &'c mut &'b mut isize = &mut *t0;
    let p: &'e isize = &**t0; //~ ERROR cannot borrow
    **t1 = 22;
}

fn main() {
}
