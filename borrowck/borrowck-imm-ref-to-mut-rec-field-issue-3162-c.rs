fn main() {
    let mut _a: u32 = 3;
    let b: &'b mut u32 = &mut _a;
    {
        let c: &'c u32 = &*b;
        _a = 4; //~ ERROR cannot assign to `_a` because it is borrowed
        drop::<&'c u32>(c);
    }
    drop::<&'b mut u32>(b);
}
