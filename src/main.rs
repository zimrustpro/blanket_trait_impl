#[derive(Debug)]
struct One;

#[derive(Debug)]
struct Two;

impl From<One> for Two {
    fn from(value: One) -> Self {
        Two 
    }
}


fn main() {
    let two: Two = One.into();
    let try_two = Two::try_from(One);
    println!("{two:?}, {:?}", try_two);
}