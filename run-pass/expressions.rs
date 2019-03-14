// FIXME: Make me pass! Diff budget: 10 lines.
// Do not `use` any items.

// Do not change the following two lines.
#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
struct IntWrapper(isize);

impl IntWrapper{
    fn max(a: IntWrapper, b: IntWrapper) -> IntWrapper{
        if a.0 < b.0{
            return b
        }
        return a
    }
}

fn max(a: isize , b: isize) -> isize{
    if a<b{
        return b
    }
    a
}

pub fn main() {
    assert_eq!(max(1usize as isize, 3), 3);
    assert_eq!(max(1u8 as isize, 3), 3);
    assert_eq!(max(1u8 as isize, 3), 3);
    assert_eq!(IntWrapper::max(IntWrapper(120), IntWrapper(248)), IntWrapper(248));
}
