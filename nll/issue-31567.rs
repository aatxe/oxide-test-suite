// Regression test for #31567: cached results of projections were
// causing region relations not to be enforced at all the places where
// they have to be enforced.

#![feature(nll)]

struct Box<T>(T);

struct VecWrapper<'a>(&'a mut SS);

struct SS(Box<u32>);

fn get_dangling<'a>(v: VecWrapper<'a>) -> &'a u32 {
    let s_inner: &'a SS = &*v.0; //~ ERROR borrow may still be in use when destructor runs [E0713]
    let res: &'a u32 = &((*s_inner).0).0;
    let tmp0: &'a mut VecWrapper<'a> = &mut v;
    drop_wrapper::<'a>(tmp0);
    res
}

fn drop_wrapper<'a>(wrapper: &'a mut VecWrapper<'a>) {
    *self.0 = SS(Box(0));
}

fn main() {
    let mut s: SS = SS(Box(11));
    let tmp0: &'t0 mut SS = &mut s;
    let vw: VecWrapper<'t0> = VecWrapper(tmp0);
    let dangling: &'t0 u32 = get_dangling::<'t0>(vw);
}
