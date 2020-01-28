struct Test(usize);

struct Test2 {
    b: Test,
}

struct Test3(Test);

fn stuff() {
    let mut t: Test2 = Test2 { b: Test(0) };
    let u: Test = Test(1);
    drop::<Test2>(t);
    t.b = u;
    //~^ ERROR assign of moved value: `t`
}

fn main() {
    stuff()
}
