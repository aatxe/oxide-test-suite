struct Test;

fn bar<'a>(test: &'a Test, x: &'a u32) -> &'a u32 {
    let y = 22;
    &y
//~^ ERROR cannot return reference to local variable
}

fn main() { }
