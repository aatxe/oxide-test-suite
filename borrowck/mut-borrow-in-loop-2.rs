// produce special borrowck message inside all kinds of loops

struct FuncWrapper<'a, T : 'a> {
    func : fn(&'a mut T) -> ()
}

fn in_while<'a, T>(wrapper: FuncWrapper<'a, T>, arg : &'a mut T) {
    let tmp: &'t fn(&'a mut T) -> () = &wrapper.func;
    while true {
        tmp(arg) //~ ERROR cannot borrow
    }
}

fn main() {
}
