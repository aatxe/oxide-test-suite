struct UnitRange([u32; 1]);

fn main() {
    let range: UnitRange = UnitRange([0]);
    let r: UnitRange = range;
    let x: u32 = range.0[0];
}
