// Test messages where a closure capture conflicts with itself because it's in
// a loop.

// a dummy string definition since any non-copyable data will do
struct String();

fn repreated_move(x: String) {
    for i in 0..10 {
        || x; //~ ERROR
    }
}

fn main() {}
