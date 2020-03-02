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

fn have_to_suggest_ref() {
    let x = B::V(String::new());
    match x {
    //~^ ERROR
        B::V(s) => drop(s),
        B::U(D(s)) => (),
    };
}
