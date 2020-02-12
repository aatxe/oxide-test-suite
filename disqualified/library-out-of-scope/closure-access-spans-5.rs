// check that accesses due to a closure capture give a special note

fn closure_move_capture_conflict(mut x: String) {
    let r = &x;
    || x; //~ ERROR
    r.use_ref();
}

fn main() {}

fn use_mut<'a, T>(x: &'a mut T) { } fn use_ref<'a, T>(x: &'a T) { }
