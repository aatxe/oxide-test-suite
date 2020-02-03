fn main() {
    let f: fn() -> usize = {
        let a: usize = 0;
        let x: &'x u32 = &a;
        || *x
    };
    f();
}
