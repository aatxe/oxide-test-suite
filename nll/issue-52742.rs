#![feature(in_band_lifetimes)]

struct Foo<'a, 'b> {
    x: &'a u32,
    y: &'b u32,
}

struct Bar<'b> {
    z: &'b u32,
}

fn take_bar<'a, 'b, 'c>(foo: Foo<'a, 'b>, b: Bar<'c>) {
    foo.y = b.z
    //~^ ERROR
}

fn main() {}
