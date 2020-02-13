// check that liveness due to a closure capture gives a special note

fn use_as_borrow_mut_capture(mut x: i32) {
    let y: &'y mut i32 = &mut x;
    x = 0; //~ ERROR
    let tmp0: &'t0 mut &'y mut i32 = &mut y;
    || **tmp0 = 1;
}

fn main() {}
