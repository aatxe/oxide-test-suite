fn gimme<'a>(x: &'a (u32,)) -> &'a u32 {
    #[lft = "t"] &(*x).0
}

fn foo<'a>(x: &'a (u32,)) -> &'a u32 {
    let v = 22;
    let tmp0 = (v,);
    let tmp1 = #[lft="t1"] &tmp0;
    gimme::<'t1>(tmp1)
    //~^ ERROR cannot return value referencing temporary value [E0515]
}

fn main() {}
