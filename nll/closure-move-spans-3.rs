// check that moves due to a closure capture give a special note

// dummy String struct, since any non-copyable type will do here
struct String();

fn borrow_mut_after_move(mut x: String) {
    || x;
    let y: &'y mut String = &mut x; //~ ERROR
}

fn main() {}
