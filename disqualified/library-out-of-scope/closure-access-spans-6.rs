// check that accesses due to a closure capture give a special note

fn closure_imm_capture_moved(mut x: String) {
    let r = x;
    || x.len(); //~ ERROR
}

fn main() {}

fn use_mut<'a, T>(x: &'a mut T) { } fn use_ref<'a, T>(x: &'a T) { }
