struct String;
struct AA(String);

fn suggest_borrow5() {
    let tmp0: AA = AA(String());
    let a: &'a AA = &tmp0;
    let s: String = (*a).0;
    //~^ ERROR
}
