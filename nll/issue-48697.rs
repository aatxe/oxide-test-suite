// Regression test for #48697

fn foo<'a>(x: &'a i32) -> &'a i32 {
    let z: i32 = 4;
    let tmp0: fn(&'y i32) -> &'y i32 = |y: &'y i32| y;
    let f: &'t0 fn(&'y i32) -> &'y i32 = &tmp0;
    let tmp1: &'t1 i32 = &z;
    let tmp2: fn(&'y i32) -> &'y i32 = *f;
    let k: &'k i32 = tmp2(tmp1);
    tmp2(x) //~ cannot return value referencing local variable
}

fn main() {}
