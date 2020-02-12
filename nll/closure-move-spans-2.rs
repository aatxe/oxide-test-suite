// check that moves due to a closure capture give a special note

// dummy String struct, since any non-copyable type will do here
struct String();

fn borrow_after_move(x: String) {
    || x;
    let y: &'y String = &x; //~ ERROR
}

fn main() {}
