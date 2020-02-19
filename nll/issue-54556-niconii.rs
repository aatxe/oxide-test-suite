// This is a reduction of a concrete test illustrating a case that was
// annoying to Rust developer niconii (see comment thread on #21114).
//
// With resolving issue #54556, pnkfelix hopes that the new diagnostic
// output produced by NLL helps to *explain* the semantic significance
// of temp drop order, and thus why inserting a semi-colon after the
// `if let` expression in `main` works.

struct Mutex;
struct MutexGuard<'a>(&'a Mutex);

fn drop_mutex<'a>(mutex: &'a mut Mutex) { }
fn drop_guard<'a>(guard: &'a mut MutexGuard<'a>) { }

fn lock<'a>(mutex: &'a Mutex) -> MutexGuard<'a> {
    MutexGuard::<'a>(mutex)
}

fn main() {
    let counter: Mutex = Mutex();

    let tmp0: &'t0 Mutex = &counter;
    let guard: MutexGuard<'t0> = lock::<'t0>(tmp0); //~ ERROR does not live long enough
    #[drop] tmp0; // temporary dies, but guard still has the loans

    let tmp1: &'t1 mut Mutex = &mut counter;
    drop_mutex::<'t1>(tmp1);
    let tmp2: &'t2 mut MutexGuard<'t0> = &mut counter;
    drop_guard(tmp2)

    // With this code as written, the dynamic semantics here implies
    // that `Mutex::drop` for `counter` runs *before*
    // `MutexGuard::drop`, which would be unsound since `MutexGuard`
    // still has a reference to `counter`.
    //
    // The goal of #54556 is to explain that within a compiler
    // diagnostic.
}
