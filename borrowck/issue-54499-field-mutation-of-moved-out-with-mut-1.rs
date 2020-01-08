#![warn(unused)]
#[derive(Debug)]
struct Sn(i32);

struct Tpair(Sn, i32);
struct Spair { x: Sn, y: i32 }

fn main() {
    {
        let mut t: (Sn, i32) = (Sn(0), 0);
        drop::<(Sn, i32)>(t);
        t.0 = Sn(1);
        //~^ ERROR assign to part of moved value
        t.1 = 2;
        // println!("{:?} {:?}", t.0, t.1);
    }
}
