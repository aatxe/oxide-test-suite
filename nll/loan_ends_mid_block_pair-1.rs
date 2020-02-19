#![allow(warnings)]
#![feature(rustc_attrs)]


fn main() {
}

fn nll_fail() {
    let mut data: (u32, u32, u32) = (0, 1, 2);
    let c: &'c mut u32 = &mut data.0;
    capitalize::<'c>(c);
    data.0 = 3;
    //~^ ERROR [E0506]
    data.0 = 4;
    data.0 = 5;
    capitalize::<'c>(c);
}

fn capitalize<'a>(_: &'a mut u32) {
}
