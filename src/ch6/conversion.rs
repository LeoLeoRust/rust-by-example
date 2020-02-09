use std::convert::{TryFrom, TryInto};
use std::fmt;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    // add code here
    fn from(item: i32) -> Self{
        Number{value: item}
    }
}

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error>{
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

//convert to string
struct Circle{
    radius: i32,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

pub fn test_conver() {
    let my_str = "hello";
    let my_string = String::from(my_str);

    //for our own type
    let number = Number::from(32);
    println!("My number is {:?}", number);

    let item = 44;
    let number: Number = item.into();
    println!("My number is {:?}", number);

    // TryFrom
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryInto
    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));

    //convert to string
    let parsed: i32 = "32".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();
    println!("parsed num are: {}, {}", parsed, turbo_parsed);
    let  circle = Circle{radius: parsed + turbo_parsed};
    println!("{}", circle.to_string());
}