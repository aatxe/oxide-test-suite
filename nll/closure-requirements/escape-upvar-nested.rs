// As in `escape-upvar-ref.rs`, test closure that:
//
// - captures a variable `y`
// - stores reference to `y` into another, longer-lived spot
//
// except that the closure does so via a second closure.

// compile-flags:-Zborrowck=mir -Zverbose

#![feature(rustc_attrs)]

#[rustc_regions]
fn test() {
    let x = 44;
    let mut p = #[lft="p"] &x;

    {
        let y = 22;

        let tmp0 = &mut p;
        let tmp1 = &y;
        let mut closure = || {
            let mut closure1 = || *tmp0 = tmp1; //~ ERROR `y` does not live long enough [E0597]
            closure1();
        };

        closure();
    }

    deref::<'p>(p);
}

fn deref<'a>(_p: &'a i32) { }

fn main() { }
