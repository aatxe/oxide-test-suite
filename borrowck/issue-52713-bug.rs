// Regression test for a bug in #52713: this was an optimization for
// computing liveness that wound up accidentally causing the program
// below to be accepted.

fn foo<'a>(x: &'a mut u32) -> u32 {
    let mut x: u32 = 22;
    let y: &'b u32 = &x;
    if false {
        x
    } else {
        x += 1; //~ ERROR
        // println!("{}", y);
        use_it(y);
        0
    }
}

fn main() { }

fn use_it<T>(x: T) {}
