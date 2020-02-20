fn gimme<'a>(x: &'a (u32,)) -> &'a u32 {
    #[lft = "a"] &(*x).0
}

fn foo<'a>(x: &'a (u32,)) -> &'a u32 {
    let v: u32 = 22;
    let tmp0: (u32,) = (v,);
    let tmp1: &'t1 (u32,) = &tmp0;
    gimme::<'t1>(tmp1)
    //~^ ERROR cannot return value referencing temporary value [E0515]
}

fn main() {}
