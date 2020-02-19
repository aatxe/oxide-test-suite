struct A(String);
struct C(D);

enum B {
    V(String),
    U(D),
}

struct D(String);

impl Drop for D {
    fn drop(&mut self) {}
}

struct F(String, String);

impl Drop for F {
    fn drop(&mut self) {}
}

fn two_separate_errors() {
    let x = (D(String::new()), &String::new());
    match x {
    //~^ ERROR
    //~^^ ERROR
        (D(s), &t) => (),
        _ => (),
    }
}
