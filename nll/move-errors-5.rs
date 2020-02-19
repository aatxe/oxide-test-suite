struct String;
struct AA(String);

fn suggest_borrow4() {
    let tmp0: [AA; 1] = [AA(String())];
    let a: AA = tmp0[0];
    //~^ ERROR
}
