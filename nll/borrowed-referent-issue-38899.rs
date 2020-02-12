// Regression test for issue #38899

#![feature(nll)]

pub struct Block<'a> {
    current: &'a u8,
    unrelated: &'a u8,
}

fn bump<'a, 'b>(mut block: &'b mut Block<'a>) {
    let x: &'x mut Block<'a> = &mut *block;
    let p: &'a u8 = &(*block).current;
    //~^ ERROR cannot borrow `*block.current` as immutable because it is also borrowed as mutable
    drop::<&'x mut Block<'a>>(x);
    drop::<&'a u8>(p);
}

fn main() {}
