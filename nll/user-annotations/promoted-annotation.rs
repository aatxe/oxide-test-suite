// Test that type annotations are checked in promoted constants correctly.

fn foo<'a>() {
    let x = 0;
    let f = |x: &'a i32| drop::<&'a i32>(x);
    f(&x);
    //~^ ERROR `x` does not live long enough
}

fn main() {}
