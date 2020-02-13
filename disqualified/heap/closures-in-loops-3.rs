// Test messages where a closure capture conflicts with itself because it's in
// a loop.

fn repreated_unique_borrow(x: &mut String) {
    let mut v = Vec::new();
    for i in 0..10 {
        v.push(|| *x = String::new()); //~ ERROR
    }
}

fn main() {}
