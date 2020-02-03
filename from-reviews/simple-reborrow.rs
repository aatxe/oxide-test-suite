struct Point(u32, u32);

fn reborrow<'a>(pt: &'a mut Point) -> &'a mut u32 {
    let r: &'a mut u32 = &mut (*pt).0;
    r
}
