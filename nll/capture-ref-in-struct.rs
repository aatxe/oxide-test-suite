// Test that a structure which tries to store a pointer to `y` into
// `p` (indirectly) fails to compile.

struct SomeStruct<'a, 'b: 'a> {
    p: &'a mut &'b i32,
    y: &'b i32,
}

fn test() {
    let x: i32 = 44;
    let mut p: &'p i32 = &x;

    {
        let y: i32 = 22;

        let tmp0: &'t0 mut &'p i32 = &mut p;
        let tmp1: &'t1 i32 = &y;
        let closure: SomeStruct<'t0, 't1> = SomeStruct::<'t0, 't1> {
            p: tmp0,
            y: tmp1,
            //~^ ERROR `y` does not live long enough [E0597]
        };

        invoke::<'t0, 't1>(closure);
    }

    deref::<'p>(p);
}

fn invoke<'a, 'b>(data: SomeStruct<'a, 'b>) where 'b: 'a {
    *data.p = data.y;
}

fn deref<'a>(_: &'a i32) { }

fn main() { }
