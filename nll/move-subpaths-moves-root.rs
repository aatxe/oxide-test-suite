// wrap it to prevent copying
struct Wrap([u32; 3]);

fn main() {
    let x: (Wrap,) = (Wrap([1, 2, 3]), );
    drop::<Wrap>(x.0);
    drop::<(Wrap,)>(x); //~ ERROR use of moved value
}
