fn foobar<'a>(x: &'a u32) -> &'a u32 {
    let y: u32 = 22;
    &y
//~^ ERROR cannot return reference to local variable
}

fn main() { }
