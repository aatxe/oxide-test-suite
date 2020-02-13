// check that liveness due to a closure capture gives a special note

fn use_as_move_capture(mut x: i32) {
    let y: &'y i32 = &x;
    x = 0; //~ ERROR
    move || *y;
}

fn main() {}
