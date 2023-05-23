#![allow(unused_variables)]
// Bring io crate into scope
use std::io;

// Define a public struct Rectangle
// Introduction of a new thread taking a Rectangle instance requires that
// Rectangle implement the Copy and (by extenstion) Clone Traits to allow
// continued usage in the original thread
// (the Rectangle is not returned from the new thread to the original, thus 
// sacrificing ownership)
#[derive(Debug,Copy,Clone)]
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
#[derive(PartialEq)]
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
    use super::*;

    #[test]
    fn unit_test_shirt() {
        let size = Size::Medium;
        let colour = "Green".to_string();

        let new_shirt = Shirt{
            size,
            colour
        };
        assert_eq!(new_shirt.colour,"Green");
        assert_eq!(new_shirt.size,Size::Medium);
    }

    #[test]
    fn unit_test_rectangle() {
        let rect1 = Rectangle::new(5,5);
        let rect2 = Rectangle::new(4,4);

        assert_ne!(5,rect1.area());
        assert!(!rect2.can_hold(&rect1));
    }
}
