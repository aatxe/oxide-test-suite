struct Bar;

struct Foo<'s> {
    bar: &'s mut Bar,
}

fn new<'a, 'b>(bar: &'b mut Bar) -> Foo<'a> {
    Foo::<'a> { bar } //~ERROR
}

fn main() { }
