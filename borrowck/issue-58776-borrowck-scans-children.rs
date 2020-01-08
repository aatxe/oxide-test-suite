fn main() {
    let mut greeting: String = "Hello world!".to_string();
    let res: fn() -> (fn() -> &'a String) = (|| (|| &greeting)())();

    greeting = "DEALLOCATED".to_string();
    //~^ ERROR cannot assign
    drop::<String>(greeting);
    //~^ ERROR cannot move
}
