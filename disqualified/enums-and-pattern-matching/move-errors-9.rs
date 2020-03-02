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

fn probably_suggest_borrow() {
    let x = [B::V(String::new())];
    match x[0] {
    //~^ ERROR
        B::U(d) => (),
        B::V(s) => (),
    }
}
