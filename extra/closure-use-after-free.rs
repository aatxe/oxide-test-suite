fn main() {
    let f = {
        let a = 0;
        let x = &a;
        || *x
    };
    f();
}
