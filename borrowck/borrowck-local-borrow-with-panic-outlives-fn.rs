fn cplusplus_mode_exceptionally_unsafe<'a>(x: &'a mut Option<&'static mut isize>) {
    let mut z: (isize, isize) = (0, 0);
    *x = Some(&mut z.1);
    //~^ ERROR `z.1` does not live long enough [E0597]
    panic!("catch me for a dangling pointer!")
}

fn main() {
    cplusplus_mode_exceptionally_unsafe(&mut None);
}
