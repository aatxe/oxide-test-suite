// ignore-tidy-linelength

struct Bar;

fn bar<'a, F: Fn()>(bar: &'a mut Bar, f: F) {}

struct Foo {
    thing: Bar,
    number: usize,
}

fn foo<'a>(foo: &'a mut Foo) {
    let tmp0: &'t0 mut Bar = &mut (*foo).thing;
    let tmp1: &'t1 &'a mut Foo = &foo;
    let cls: fn() = || {
    //~^ ERROR cannot borrow `self.thing` as mutable because it is also borrowed as immutable [E0502]
        (**tmp1).number;
    };
    #[envs(cls)] bar::<'t0>(tmp0, cls);
}

fn main() {}
