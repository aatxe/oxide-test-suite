struct Sty(i32);

struct Tpair(Sty, i32);
struct Spair { x: Sty, y: i32 }

fn main() {
        let u: Tpair = Tpair(Sty(0), 0);
        drop::<Tpair>(u);
        u.0 = Sty(1);
        //~^ ERROR assign to part of moved value: `u` [E0382]
        //~| ERROR cannot assign to `u.0`, as `u` is not declared as mutable [E0594]
        u.1 = 2;
        //~^ ERROR cannot assign to `u.1`, as `u` is not declared as mutable [E0594]
}
