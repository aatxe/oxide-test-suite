// check that existing borrows due to a closure capture give a special note

fn move_while_borrowed(x: String) {
    let f = || x.len();
    let y = x; //~ ERROR
    use_ref(f);
}

fn main() {}

fn use_mut<'a, T>(x: &'a mut T) { } fn use_ref<'a, T>(x: &'a T) { }
