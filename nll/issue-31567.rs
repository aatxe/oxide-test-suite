// Regression test for #31567: cached results of projections were
// causing region relations not to be enforced at all the places where
// they have to be enforced.

#![feature(nll)]

struct Box<T>(T);

struct VecWrapper<'a>(&'a mut SS);

struct SS(Box<u32>);

fn get_dangling<'a>(v: VecWrapper<'a>) -> &'a u32 {
    let s_inner = &*v.0; //~ ERROR borrow may still be in use when destructor runs [E0713]
    let res = #[lft = "res"] &((*s_inner).0).0;
    let tmp0 = #[lft = "t0"] &mut v;
    drop_wrapper::<'t0, 'res>(tmp0);
    res
}

fn drop_wrapper<'o, 'i>(wrapper: &'o mut VecWrapper<'i>) where 'i: 'o {
    *(*wrapper).0 = SS(Box::<u32>(0));
}

fn main() {
    let mut s = SS(Box::<u32>(11));
    let tmp0 = #[lft="t0"] &mut s;
    let vw = VecWrapper::<'t0>(tmp0);
    let dangling = get_dangling::<'t0>(vw);
}
