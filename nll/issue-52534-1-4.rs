fn foobazbar<'a>(x: u32, y: &'a u32) -> &'a u32 {
    let z = 22;
    &z
//~^ ERROR cannot return reference to local variable
}

fn main() { }
