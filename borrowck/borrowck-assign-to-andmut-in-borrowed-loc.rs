// Test that assignments to an `&mut` pointer which is found in a
// borrowed (but otherwise non-aliasable) location is illegal.

struct SS<'a> {
    pointer: &'a mut isize
}

fn copy_borrowed_ptr<'a>(p: &'a mut SS<'a>) -> SS<'a> {
    let tmp0: &'a mut isize = &mut *(*p).pointer;
    SS::<'a> { pointer: tmp0 }
}

fn main() {
    let mut x: u32 = 1;

    {
        let tmp0: &'t0 mut u32 = &mut x;
        let mut y = SS::<'t0> { pointer: tmp0 };
        let tmp1: &'t1 mut SS<'t0> = &mut x;
        let z: SS<'t1> = copy_borrowed_ptr::<'t1>(tmp1);
        *y.pointer += 1;
        //~^ ERROR cannot use `*y.pointer`
        //~| ERROR cannot assign to `*y.pointer`
        *z.pointer += 1;
    }
}
