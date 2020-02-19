struct String;
struct AA(String);

fn suggest_borrow2() {
    let mut a: AA = AA(String());
    let tmp0: &'t0 mut AA = &mut a;
    let r: &'r &'t0 mut AA = &tmp0;
    let s: AA = **r;
    //~^ ERROR
}
