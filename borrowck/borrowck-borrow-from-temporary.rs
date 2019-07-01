// Test lifetimes are linked properly when we take reference
// to interior.

fn id<T>(x: T) -> T { x }

struct Foo(isize);

fn foo<'a>() -> &'a isize {
    let tmp: Foo = id::<Foo>(Foo(3));
    let x: &'x isize = &tmp.0;
    x //~ ERROR cannot return value referencing temporary value
}

pub fn main() {
}
