// produce special borrowck message inside all kinds of loops

struct FuncWrapper<'a, T : 'a> {
    func : fn(&'a mut T) -> ()
}

fn in_loop<'a, T>(wrapper: FuncWrapper<'a, T>, arg : &'a mut T) {
    loop {
        let tmp: fn(&'a mut T) -> () = wrapper.func;
        tmp(arg) //~ ERROR cannot borrow
    }
}

fn main() {
}
