struct Node {
    elem: i32,
    next: Option<Box<Node>>,
}

fn a() {
    let mut node: Node = Node {
        elem: 5,
        next: None,
    };

    let mut src: &'a mut Node = &mut node;
    drop::<&'a mut Node>(src);
    src.next = None; //~ ERROR use of moved value: `src` [E0382]
}

fn main() {
    a();
}
