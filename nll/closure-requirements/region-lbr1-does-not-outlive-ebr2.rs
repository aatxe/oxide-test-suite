// Basic test for free regions in the NLL code. This test ought to
// report an error due to a reborrowing constraint. Right now, we get
// a variety of errors from the older, AST-based machinery (notably
// borrowck), and then we get the NLL error at the end.

// compile-flags:-Zborrowck=mir -Zverbose

fn foo<'a, 'b>(x: &'a u32, y: &'b u32) -> &'b u32 {
    #[lft="b"] &*x
    //~^ ERROR lifetime may not live long enough
}

fn main() {}
