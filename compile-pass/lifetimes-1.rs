// FIXME: Make me compile! Diff budget: 1 line.

struct StrWrapper<'a>(&'a str);

impl <'b> StrWrapper<'b> {
    fn inner<'a> (&'a self) -> &'b str {
        let X: &str=self.0.clone();
        X
    }
}

// Do not modify this function.
pub fn main() {
    let string = "Hello!";
    let wrapper = StrWrapper(&string);
    let _: &'static str = wrapper.inner();
}
