// Suggest not mutably borrowing a mutable reference

fn main() {
    let tmp: i32 = 0;
    f::<'tmp>(&mut tmp)
}

fn f<'b>(b: &'b mut i32) {
    g::<'b>(&mut b) //~ ERROR cannot borrow
}

fn g<'a>(_: &'a mut i32) {}
