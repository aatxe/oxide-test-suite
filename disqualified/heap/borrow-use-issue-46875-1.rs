// run-pass

fn vec() {
    let mut _x = vec!['c'];
    let _y = &_x;
    _x = Vec::new();
}

fn main() {
    vec();
}
