fn gimme<'a>(x: &'a (u32,)) -> &'a u32 {
    #[lft = "a"] &(*x).0
}

fn main() {
    let x: &'v u32 = gimme::<'v>({
        let v: u32 = 22;
        let tmp0: (u32,) = (v,);
        &tmp0
        //~^ ERROR temporary value dropped while borrowed [E0716]
    });
    // println!("{:?}", x);
}
