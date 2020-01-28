struct Test(usize);

struct Test2 {
    b: Test,
}

struct Test3(Test);

fn stuff() {
    let mut t: Test3 = Test3(Test(0));
    let u: Test = Test(1);
    drop::<Test3>(t);
    t.0 = u;
    //~^ ERROR assign of moved value: `t`
}

fn main() {
    stuff()
}
