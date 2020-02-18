// Tests that two closures cannot simultaneously have mutable
// access to the variable, whether that mutable access be used
// for direct assignment or for taking mutable ref. Issue #6801.

fn to_fn_mut<F: FnMut()>(f: F) -> F { f }

fn a() {
    let mut x: usize = 3;
    let tmp0: &'t0 mut usize = &mut x;
    let c1_tmp: fn() = || *tmp0 = 4;
    let c1: fn() = #[envs(c1_tmp)] to_fn_mut(c1_tmp);
    let tmp1: &'t1 mut usize = &mut x;
    let c2_tmp: fn() = || *tmp1 = 5;
    let c2: fn() = #[envs(c2_tmp)] to_fn_mut(c2_tmp); //~ ERROR cannot borrow `x` as mutable more than once
    drop::<(fn(), fn())>((c1, c2));
}

fn main() {
}
