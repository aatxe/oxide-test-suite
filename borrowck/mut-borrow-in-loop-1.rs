// produce special borrowck message inside all kinds of loops

struct FuncWrapper<'a, T : 'a> {
    func : fn(&'a mut T) -> ()
}

fn in_loop<'a, T>(wrapper: FuncWrapper<'a, T>, arg : &'a mut T) {
    let tmp: &'t0 fn(&'a mut T) -> () = &wrapper.func;
    loop {
        tmp(arg) //~ ERROR cannot borrow
    }
}

fn main() {
}
