// FIXME: Make me pass! Diff budget: 30 lines.

struct Builder {
    string: Option<String>,
    number: Option<usize>,    
}

impl Builder{
    fn default() -> Self{
        Builder{
            string: None,
            number: None,            
        }
    }
    fn clone(&self) -> Self{
        Builder{
            string: self.string.clone(),
            number: self.number.clone(),
        }
    }
    fn to_string(&self) -> String{
        let mut info: String = String::new();
        match self.string{
            Some(ref x) => {
                info.push_str(x);
                info.push_str(" ");
                },
            None => info.push_str(&"".to_owned())
        }
        match self.number{
            Some(x) => info.push_str(&x.to_string()),
            None => info.push_str(&"".to_owned())
        }
        String::from(info.trim())
    }
    fn string(&mut self, word: &str) -> Self{
        self.string = Some(String::from(word));                
        self.clone()
    }
    fn number(&mut self, x: usize) -> Self{
        self.number = Some(x);
        self.clone()
    }
}

// Do not modify this function.
fn main() {
    let empty = Builder::default().to_string();
    assert_eq!(empty, "");
    
    let just_str = Builder::default().string("hi").to_string();
    assert_eq!(just_str, "hi");

    let just_num = Builder::default().number(254).to_string();
    assert_eq!(just_num, "254");
    
    let a = Builder::default()
        .string("hello, world!")
        .number(200)
        .to_string();

    assert_eq!(a, "hello, world! 200");
    
    let b = Builder::default()
        .string("hello, world!")
        .number(200)
        .string("bye now!")
        .to_string();

    assert_eq!(b, "bye now! 200");
    
    let c = Builder::default()
        .string(&"heap!".to_owned())
        .to_string();

    assert_eq!(c, "heap!");
    
}
