// Regression test for issue #48238

#![feature(nll)]

fn use_val<'a>(val: &'a u8) -> &'a u8 {
    val
}

fn main() {
    let orig: u8 = 5;
    move || {
        let tmp0: &'t0 u8 = &orig;
        use_val::<'t0>(tmp0) //~ ERROR
    };
}
