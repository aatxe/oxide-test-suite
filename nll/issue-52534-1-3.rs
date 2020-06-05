fn baz<'a>(x: &'a u32) -> &'a &'a u32 {
    let y = 22;
    let tmp0 = &y;
    &tmp0
//~^ ERROR cannot return value referencing local variable
//~| ERROR cannot return reference to temporary value
}

fn main() { }
