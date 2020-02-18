fn expect_fn<F: Fn()>(f: F) {
    f();
}

fn main() {
   {
       let x: ([u32; 1], [u32; 1]) = ([22], [44]);
       let tmp0: &'t0 mut ([u32; 1], [u32; 1]) = &mut x;
       let cls: fn() = || drop::<[u32; 1]>((*tmp0).0);
       #[envs(cls)] expect_fn(cls);
       //~^ ERROR cannot move out of `x.0`, as `x` is a captured variable in an `Fn` closure [E0507]
   }
}
