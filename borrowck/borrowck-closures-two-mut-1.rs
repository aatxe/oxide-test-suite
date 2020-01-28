// Tests that two closures cannot simultaneously have mutable
// access to the variable, whether that mutable access be used
// for direct assignment or for taking mutable ref. Issue #6801.

fn to_fn_mut(f: fn() -> ()) -> fn() -> () { f }

fn a() {
    let mut x: usize = 3;
    let c1: fn() -> () = to_fn_mut(|| x = 4);
    let c2: fn() -> () = to_fn_mut(|| x = 5); //~ ERROR cannot borrow `x` as mutable more than once
    drop::<(fn() -> (), fn() -> ())>((c1, c2));
}

fn main() {
}
