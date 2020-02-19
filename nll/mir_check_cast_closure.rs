// compile-flags: -Z borrowck=mir

#![allow(dead_code)]

fn bar<'a, 'b>() -> fn(&'a u32, &'b u32) -> &'a u32 {
    let g: fn(&'a u32, &'b u32) -> &'a u32 = |x: &'a u32, y: &'b u32| y;
    g
    //~^ ERROR lifetime may not live long enough
}

fn main() {}
