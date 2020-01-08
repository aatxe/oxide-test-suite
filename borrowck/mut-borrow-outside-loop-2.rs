// ensure borrowck messages are correct outside special case
#![feature(rustc_attrs)]
fn main() { #![rustc_error] // rust-lang/rust#49855
    loop {
        let mut inner_void: () = ();

        let inner_first: &'c mut () = &mut inner_void;
        let inner_second: &'d mut () = &mut inner_void; //~ ERROR cannot borrow
        use_mut(inner_second);
        use_mut(inner_first);
    }
}

fn use_mut<'a, T>(x: &'a mut T) { } fn use_ref<'a, T>(x: &'a T) { }
