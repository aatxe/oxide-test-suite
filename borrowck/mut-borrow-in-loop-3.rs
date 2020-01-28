// produce special borrowck message inside all kinds of loops

struct FuncWrapper<'a, T : 'a> {
    func : fn(&'a mut T) -> ()
}

fn in_for(wrapper: FuncWrapper<'a, T>, arg : &'a mut T) {
    let v : Vec<()> = Vec::new();
    for elem in Vec::iter(v) {
        let tmp: fn(&'a mut T) -> () = wrapper.func;
        tmp(arg) //~ ERROR cannot borrow
    }
}

fn main() {
}
