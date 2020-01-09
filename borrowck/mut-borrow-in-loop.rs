// produce special borrowck message inside all kinds of loops

struct FuncWrapper<'a, T : 'a> {
    func : fn(&'a mut T) -> ()
}

fn in_loop<'a, T>(wrapper: FuncWrapper<'a, T>, arg : &'a mut T) {
    loop {
        (wrapper.func)(arg) //~ ERROR cannot borrow
    }
}

fn in_while<'a, T>(wrapper: FuncWrapper<'a, T>, arg : &'a mut T) {
    while true {
        (wrapper.func)(arg) //~ ERROR cannot borrow
    }
}

fn in_for(wrapper: FuncWrapper<'a, T>, arg : &'a mut T) {
    let v : Vec<()> = vec![];
    for _ in v.iter() {
        (wrapper.func)(arg) //~ ERROR cannot borrow
    }
}

fn main() {
}
