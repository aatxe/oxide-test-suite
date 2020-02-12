// run-pass

fn int() {
    let mut _x = 5;
    let _y = &_x;
    _x = 7;
}

fn main() {
    int();
}
