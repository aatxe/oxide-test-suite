// Test closure that:
//
// - captures a variable `y` by reference
// - stores that reference to `y` into another, longer-lived place (`p`)
//
// Both of these are upvars of reference type (the capture of `y` is
// of type `&'a i32`, the capture of `p` is of type `&mut &'b
// i32`). The closure thus computes a relationship between `'a` and
// `'b`.  This relationship is propagated to the closure creator,
// which reports an error.

// compile-flags:-Zborrowck=mir -Zverbose

#![feature(rustc_attrs)]

#[rustc_regions]
fn test() {
    let x = 44;
    let mut p = #[lft = "p"] &x;

    {
        let y = 22;
        let tmp0 = &mut p;
        let tmp1 = &y;
        let mut closure = || *tmp0 = tmp1;
        //~^ ERROR `y` does not live long enough [E0597]
        closure();
    }

    deref::<'p>(p);
}

fn deref<'a>(_p: &'a i32) { }

fn main() { }
