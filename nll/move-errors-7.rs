struct String;
struct CC(DD);

fn suggest_ref() {
    let c: CC = CC(DD(String()));
    let tmp0: DD = c.0;
    let s: String = tmp0.0;
    let tmp1: &'t1 mut DD = &mut tmp0;
    drop_d::<'t1>(tmp1);
    #[drop] tmp0;
    //~^ ERROR
}

struct DD(String);

fn drop_d<'a>(d: &'a mut DD) {}
