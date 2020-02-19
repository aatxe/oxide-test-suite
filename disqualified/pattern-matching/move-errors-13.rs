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

fn double_binding(x: &Result<String, String>) {
    match *x {
    //~^ ERROR
        Ok(s) | Err(s) => (),
    }
}
