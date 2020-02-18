fn baz<'a>(x: &'a u32) -> &'a &'a u32 {
    let y: u32 = 22;
    let tmp0: &'t0 u32 = &y;
    &tmp0
//~^ ERROR cannot return value referencing local variable
//~| ERROR cannot return reference to temporary value
}

fn main() { }
