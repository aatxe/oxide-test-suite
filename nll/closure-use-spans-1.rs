// check that liveness due to a closure capture gives a special note

fn use_as_borrow_capture(mut x: i32) {
    let y: &'y i32 = &x;
    x = 0; //~ ERROR
    let tmp0: &'t0 &'y i32 = &y;
    || **tmp0;
}

fn main() {}
