fn main() {
    let range: [u32; 1] = [0];
    let r: [u32; 1] = range;
    let x: u32 = range[0];
    // this example originally used ranges and errored, but
    // the array desugaring turns them into something that implements Copy
    //
    // we could instead produce the same behavior by making a separate Range construct
    // Ranges could be identical to Arrays but not implement Copy
}
