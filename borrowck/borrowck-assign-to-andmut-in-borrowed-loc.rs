// Test that assignments to an `&mut` pointer which is found in a
// borrowed (but otherwise non-aliasable) location is illegal.

struct SS<'a> {
    pointer: &'a mut isize
}

fn copy_borrowed_ptr<'a>(p: &'a mut SS<'a>) -> SS<'a> {
    SS::<'a> { pointer: &mut *(*p).pointer }
}

fn main() {
    let mut x: u32 = 1;

    {
        let mut y: SS<'a> = SS::<'a> { pointer: &mut x };
        let z: SS<'a> = copy_borrowed_ptr::<'a>(&mut y);
        *y.pointer += 1;
        //~^ ERROR cannot use `*y.pointer`
        //~| ERROR cannot assign to `*y.pointer`
        *z.pointer += 1;
    }
}
