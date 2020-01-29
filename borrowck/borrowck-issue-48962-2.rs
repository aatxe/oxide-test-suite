// struct Node {
//     elem: i32,
//     next: Option<Box<Node>>,
// }

fn b() {
    let mut tmp: (u32, u32) = (22, 44);
    let mut src: &'b mut (u32, u32) = &mut tmp;
    drop::<&'b mut (u32, u32)>(src);
    (*src).0 = 66; //~ ERROR use of moved value: `src` [E0382]
}

fn main() {
    b();
}
