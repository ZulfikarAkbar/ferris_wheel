// FIXME: Make me compile! Diff budget: 3 lines.

// Do not modify the inner type &'a T.
struct RefWrapper<'a, T:'a>(&'a T);

impl<'b, T: 'b> RefWrapper<'b, T> {
    fn inner<'a>(&'a self) -> &'b T {
        let x = self.0.clone();
        x
    }
}

// Do not modify this function.
pub fn main() {
    let x = 1;
    let mut r = &x;
    let z = RefWrapper(r);
    r = z.inner();
}
