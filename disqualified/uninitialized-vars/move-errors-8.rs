struct String;
struct AA(String);

fn suggest_nothing() {
    let tmp: AA = AA(String());
    let a: &'a AA = &tmp;
    let b: String;
    b = *a;
    //~^ ERROR
}

