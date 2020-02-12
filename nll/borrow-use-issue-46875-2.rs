// run-pass

fn int() {
    let mut _x: u32 = 5;
    let _y: &'y u32 = &_x;
    #[drop] _y;
    _x = 7;
}

fn main() {
    int();
}
