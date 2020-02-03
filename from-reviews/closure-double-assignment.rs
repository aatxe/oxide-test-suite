fn main() {
    let a: u32 = 0;
    let x: &'x mut u32 = &mut a;
    let f: fn() -> () = |v: u32| { *x = v };
    // $\Gamma_C$ is again empty because nothing got consumed.
    // However, `x` and the reference captured by `f` alias, so this
    // should get rejected:
    *x = 1;
    f(3);
}
