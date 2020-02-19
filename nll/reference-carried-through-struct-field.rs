struct Wrap<'a> { w: &'a mut u32 }

fn foo() {
    let mut x: u32 = 22;
    let tmp0: &'t0 mut u32 = &mut x;
    let wrapper: Wrap<'t0> = Wrap::<'t0> { w: tmp0 };
    x += 1; //~ ERROR cannot use `x` because it was mutably borrowed [E0503]
    *(wrapper.w) += 1;
}

fn main() { }
