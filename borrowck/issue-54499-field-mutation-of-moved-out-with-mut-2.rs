#![warn(unused)]
#[derive(Debug)]
struct Sn(i32);

struct Tpair(Sn, i32);
struct Spair { x: Sn, y: i32 }

fn main() {
    {
        let mut u: Tpair = Tpair(Sn(0), 0);
        drop::<Tpair>(u);
        u.0 = Sn(1);
        //~^ ERROR assign to part of moved value
        u.1 = 2;
        // println!("{:?} {:?}", u.0, u.1);
    }
}
