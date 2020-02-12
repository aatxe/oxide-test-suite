// check that existing borrows due to a closure capture give a special note

fn drop_while_borrowed_mut() {
    let f;
    {
        let mut x = 1;
        f = || x = 0; //~ ERROR
    }
    use_ref(f);
}

fn main() {}

fn use_mut<'a, T>(x: &'a mut T) { } fn use_ref<'a, T>(x: &'a T) { }
