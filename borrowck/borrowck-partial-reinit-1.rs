struct Test;

struct Test2 {
    b: Option<Test>,
}

struct Test3(Option<Test>);

fn stuff() {
    let mut t: Test2 = Test2 { b: None };
    let u: Test = Test;
    drop::<Test2>(t);
    t.b = Some(u);
    //~^ ERROR assign of moved value: `t`

    let mut t: Test3 = Test3(None);
    let u: Test = Test;
    drop::<Test3>(t);
    t.0 = Some(u);
    //~^ ERROR assign of moved value: `t`
}

fn main() {
    stuff()
}
