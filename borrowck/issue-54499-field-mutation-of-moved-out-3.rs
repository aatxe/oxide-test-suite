struct Sty(i32);

struct Tpair(Sty, i32);
struct Spair { x: Sty, y: i32 }

fn main() {
        let v: Spair = Spair { x: Sty(0), y: 0 };
        drop::<Spair>(v);
        v.x = Sty(1);
        //~^ ERROR assign to part of moved value: `v` [E0382]
        //~| ERROR cannot assign to `v.x`, as `v` is not declared as mutable [E0594]
        v.y = 2;
        //~^ ERROR cannot assign to `v.y`, as `v` is not declared as mutable [E0594]
}
