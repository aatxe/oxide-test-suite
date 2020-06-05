#![feature(nll)]

fn test<'a>() {
    let f: fn(&'f ()) = |x:&'a ()| {}; //~ ERROR lifetime may not live long enough
    //~^ ERROR lifetime may not live long enough
}

fn main() {
    test::<'t0>();
}
