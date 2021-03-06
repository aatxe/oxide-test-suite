// Some cases with closures that might be problems

// Should have one error per assignment

fn fn_ref<F: Fn()>(f: F) -> F { f }

// This still gives two messages, but it requires two things to be fixed.
fn two_closures_ref(x: i32) {
    let tmp0: &'t0 mut i32 = &mut x;
    let c0: fn() = || {
        || *tmp0 = 1;  //~ ERROR
    };
    #[envs(c0)] fn_ref(c0);
    let c1: fn() = move || {
        let tmp1: &'t1 mut i32 = &mut x;
        || *tmp1 = 1; //~ ERROR
    };
    #[envs(c1)] fn_ref(c1); //~ ERROR
}

fn main() {}
