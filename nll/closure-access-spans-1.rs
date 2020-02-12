// check that accesses due to a closure capture give a special note

fn closure_imm_capture_conflict(mut x: i32) {
    let r: &'r mut i32 = &mut x;
    || x; //~ ERROR
    use_mut::<'r, i32>(r);
}

fn main() {}

fn use_mut<'a, T>(x: &'a mut T) { } fn use_ref<'a, T>(x: &'a T) { }
