// FIXME: Make me pass! Diff budget: 2 lines.

struct Dummy;
struct Dummy2;
pub trait Foo {
    fn foo(&self) -> usize { 1 }
}

pub trait FooToo {
    fn foo(&self) -> usize { 2 }
}

impl Foo for Dummy { }

impl FooToo for Dummy2 { }

fn main() {
    let dummy = Dummy;
    let dummy2 = Dummy2;
    let x = dummy.foo();
    let y = dummy2.foo();

    // Values for `x` and `y` must come from calling `foo()` methods.
    assert_eq!(x, 1);
    assert_eq!(y, 2);
}
