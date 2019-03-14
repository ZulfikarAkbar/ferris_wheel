// FIXME: Make me compile. Diff budget: 12 line additions and 2 characters.
struct ErrorA;
struct ErrorB;

enum Error {
    A(ErrorA),
    B(ErrorB)
}

fn do_a() -> Result<u16, ErrorA> {
    Err(ErrorA)
}

fn do_b() -> Result<u32, ErrorB> {
    Err(ErrorB)
}

fn do_both() -> Result<(u16, u32), Error> {
    //Ok((do_a(), do_b()))
    let a=do_a();
    let b=do_b();
    match(a,b){
        (Ok(x), Ok(y)) => return Ok((x,y)),
        (Err(x), Ok(y)) => return Err(Error::A(x)),
        (Ok(x), Err(y)) => return Err(Error::B(y)),
        (_,_) => return Err(Error::A(ErrorA))
    }
    //Err(Error::A(ErrorA))
}

fn main() { }
