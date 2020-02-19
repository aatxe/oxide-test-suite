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

fn have_to_suggest_double_ref() {
    let x = F(String::new(), String::new());
    match x {
    //~^ ERROR
        F(s, mut t) => (),
        _ => (),
    }
}
