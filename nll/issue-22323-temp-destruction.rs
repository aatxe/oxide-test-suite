// rust-lang/rust#22323: regression test demonstrating that NLL
// precisely tracks temporary destruction order.

// compile-pass

// dummy string, any non-copyable value will do
struct String;

fn main() {
    let tmp0: Value = construct();
    let tmp1: &'t1 Value = &tmp0;
    let tmp2: Borrowed<'t1> = borrow::<'t1>(tmp1);
    let _s: String = consume_borrowed::<'t1>(tmp2);
}

fn construct() -> Value { Value() }

pub struct Value;

fn borrow<'a>(value: &'a Value) -> Borrowed<'a> { unimplemented!() }

pub struct Borrowed<'a> {
    _inner: Guard<'a, Value>,
}

fn consume_borrowed<'a>(borrowed: Borrowed<'a>) -> String { unimplemented!() }

pub struct Guard<'a, T: 'a> {
    _lock: &'a T,
}
