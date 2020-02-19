struct Point(u32, u32);

fn reborrow<'a>(pt: &'a mut Point) -> &'a mut u32 {
    let r = #[lft = "a"] &mut (*pt).0;
    r
}
