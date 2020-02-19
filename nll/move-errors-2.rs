struct String;
struct AA(String);

fn suggest_borrow() {
    let a: [AA; 1] = [AA(String())];
    let b: AA = a[0];
    //~^ ERROR
}
