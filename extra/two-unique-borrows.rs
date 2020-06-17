fn take_refs<'a, 'b>(x: &'a mut u32, y: &'b mut u32) {
    *x = 9;
    *y = 15;
}

fn main() {
    let mut x = 42;
    take_refs::<'t0, 't1>(#[lft="t0"] &mut x, #[lft="t1"] &mut x);
}
