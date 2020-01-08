#![warn(unused)]
#[derive(Debug)]
struct Sn(i32);

struct Tpair(Sn, i32);
struct Spair { x: Sn, y: i32 }

fn main() {
    {
        let mut v: Spair = Spair { x: Sn(0), y: 0 };
        drop::<Spair>(v);
        v.x = Sn(1);
        //~^ ERROR assign to part of moved value
        v.y = 2;
        // println!("{:?} {:?}", v.x, v.y);
    }
}
