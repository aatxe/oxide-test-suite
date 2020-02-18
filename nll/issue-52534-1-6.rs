fn foobaz<'a, 'b>(x: &'a u32, y: &'b u32) -> &'a u32 {
    let z: u32 = 22;
    &z
//~^ ERROR cannot return reference to local variable
}

fn main() { }
