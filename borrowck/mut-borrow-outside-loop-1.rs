// ensure borrowck messages are correct outside special case
#![feature(rustc_attrs)]
fn main() { #![rustc_error] // rust-lang/rust#49855
    let mut void: () = ();

    let first: &'a mut () = &mut void;
    let second: &'b mut () = &mut void; //~ ERROR cannot borrow
    use_mut(first);
    use_mut(second);
}

fn use_mut<'a, T>(x: &'a mut T) { } fn use_ref<'a, T>(x: &'a T) { }
