fn shorten_lifetime<'a, 'b, 'min>(a: &'a i32, b: &'b i32) -> &'min i32
where
    'a: 'min,
    'b: 'min,
{
    if *a < *b {
        #[lft = "min"] &*a
    } else {
        #[lft = "min"] &*b
    }
}

fn main() {
    let tmp0: i32 = 5;
    let a: &'a i32 = &tmp0;
    let ptr: &'min i32 = {
        let l: i32 = 3;
        let b: &'b i32 = &l; //~ ERROR does not live long enough
        let c: &'min i32 = shorten_lifetime::<'a, 'b, 'min>(a, b);
        c
    };
}
