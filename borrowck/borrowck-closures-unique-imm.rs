struct Foo {
    x: isize,
}

pub fn main() {
    let mut tmp = Foo {
        x: 1,
    };
    let mut this = &mut tmp;
    let mut r = || {
        let p = #[lft="p"] &(*this).x;
        &mut (*this).x; //~ ERROR cannot borrow
        use_ref::<'p, isize>(p);
    };
    r()
}

fn use_mut<'a, T>(x: &'a mut T) { } fn use_ref<'a, T>(x: &'a T) { }
