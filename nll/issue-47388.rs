struct FancyNum {
    num: u8,
}

fn main() {
    let mut fancy: FancyNum = FancyNum{ num: 5 };
    let tmp0: &'t0 mut FancyNum = &mut fancy;
    let fancy_ref: &'f &'t0 mut FancyNum = &tmp0;
    (**fancy_ref).num = 6; //~ ERROR E0594
}
