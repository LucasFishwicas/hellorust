#![allow(unused_variables)]
// Bring io crate into scope
use std::io;

// Define a public struct Rectangle
#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}
// Implement block for Rectangle methods
impl Rectangle {
    // Method for creating a Rectangle instance
    pub fn new(w: u32, h: u32) -> Rectangle {
        Rectangle {
            width: w,
            height: h
        }
    }

    // Method for calculating the area of the Rectangle it is called on
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    // Method for determining whether a given rectangle instance fits inside
    // the rectangle it is called on
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
// Define private enum Size intended to be used in the Shirt struct
#[derive(Debug)]
enum Size {
    Small,
    Medium,
    Large,
}
// Define a private struct Shirt
#[derive(Debug)]
struct Shirt {
    size: Size,
    colour: String,
}


//  Define a function Read() intended to take Rectangle parameters as user input
pub fn read(string: &str) -> u32 {
    let mut temp = String::new();

    //loop {
        println!("Give me the {} of your Rectangle", string);

        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

        let temp: u32 = temp.trim().parse()
            .expect("Please type a number");
    //}

    temp
}

// Initial practise for implementing unit tests
#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let result = 2+2;
        assert_eq!(result,4);
    }
}
