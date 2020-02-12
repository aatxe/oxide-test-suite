// check that moves due to a closure capture give a special note

// dummy String struct, since any non-copyable type will do here
struct String();

fn move_after_move(x: String) {
    || x;
    let y: String = x; //~ ERROR
}

fn main() {}
