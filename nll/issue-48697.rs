// Regression test for #48697

fn foo<'a>(x: &'a i32) -> &'a i32 {
    let z = 4;
    let tmp0 = |y: &'y i32| y;
    let f = &tmp0;
    let tmp1 = &z;
    let tmp2 = *f;
    let k = tmp2(tmp1);
    tmp2(x) //~ cannot return value referencing local variable
}

fn main() {}
