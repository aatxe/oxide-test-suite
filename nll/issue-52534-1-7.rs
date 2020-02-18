fn foobarbaz<'a, 'b>(x: &'a u32, y: &'b u32, z: &'a u32) -> &'a u32 {
    let w: u32 = 22;
    &w
//~^ ERROR cannot return reference to local variable
}

fn main() { }
