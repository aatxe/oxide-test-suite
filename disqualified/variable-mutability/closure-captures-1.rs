// Some cases with closures that might be problems

// Should have one error per assignment

fn one_closure(x: i32) {
    || x = 1; //~ ERROR
    move || x = 1; //~ ERROR
}

fn main() {}
