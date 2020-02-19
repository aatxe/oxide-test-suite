#![feature(nll)]

fn main() {
    let tmp: i32 = 3;
    let i: &'t0 i32 = &tmp;

    let f: fn(&'f1 i32) -> &'f2 i32 = |x: &'f1 i32| -> &'f2 i32 { x };
    //~^ ERROR lifetime may not live long enough
    let j: &'t0 i32 = f(i);

    let g: fn(&'x1 i32) -> &'x2 i32 = |x: &'x1 i32| { x };
    //~^ ERROR lifetime may not live long enough
    let k: &'t0 i32 = g(i);
}
