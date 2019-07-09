struct Sty(i32);

struct Tpair(Sty, i32);
struct Spair { x: Sty, y: i32 }

fn main() {
        let t: (Sty, i32) = (Sty(0), 0);
        drop::<(Sty, i32)>(t);
        t.0 = Sty(1);
        //~^ ERROR assign to part of moved value: `t` [E0382]
        //~| ERROR cannot assign to `t.0`, as `t` is not declared as mutable [E0594]
        t.1 = 2;
        //~^ ERROR cannot assign to `t.1`, as `t` is not declared as mutable [E0594]
}
