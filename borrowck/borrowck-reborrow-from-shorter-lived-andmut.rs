// Test that assignments to an `&mut` pointer which is found in a
// borrowed (but otherwise non-aliasable) location is illegal.

struct Sn<'a> {
    pointer: &'a mut isize
}

fn copy_borrowed_ptr<'a,'b>(p: &'a mut Sn<'b>) -> Sn<'b> {
    Sn::<'b> { pointer: &mut *(*p).pointer }
    //~^ ERROR lifetime mismatch
}

fn main() {
    let mut x: isize = 1;

    {
        let mut y: Sn<'x> = Sn::<'x> { pointer: &mut x };
        let z: Sn<'x> = copy_borrowed_ptr::<'y, 'x>(&mut y);
        *y.pointer += 1;
        *z.pointer += 1;
    }
}
