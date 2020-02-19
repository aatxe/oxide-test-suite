struct A(String);
struct C(D);

fn suggest_borrow3() {
    use std::rc::Rc;
    let mut a = A("".to_string());
    let r = Rc::new(a);
    let s = *r;
    //~^ ERROR
}
