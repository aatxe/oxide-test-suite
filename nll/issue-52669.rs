struct AA {
    b: BB,
}

#[derive(Clone, Copy)]
struct BB;

fn clone<'a>(b: &'a BB) -> BB {
    *b
}

fn foo(a: AA) {}

fn bar(mut a: AA) -> BB {
    a.b = BB();
    foo(a);
    let tmp0: &'t0 BB = &a.b;
    clone::<'t0>(tmp0)
//~^ ERROR borrow of moved value
}

fn main() {}
