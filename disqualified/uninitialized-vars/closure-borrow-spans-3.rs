// check that existing borrows due to a closure capture give a special note

fn drop_while_borrowed() {
    let f;
    {
        let x = 1;
        f = || x; //~ ERROR
    }
    use_ref(f);
}

fn main() {}

fn use_mut<'a, T>(x: &'a mut T) { } fn use_ref<'a, T>(x: &'a T) { }
