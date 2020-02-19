struct String;
struct AA(String);

fn suggest_remove_deref() {
    let tmp0: AA = AA(String());
    let a: &'t0 AA = &tmp0;
    let b: AA = *a;
    //~^ ERROR
}
