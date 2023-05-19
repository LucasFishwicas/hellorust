// Bring library lib.rs into scope
mod lib;

// Define main() function which will run when cargo run is called
fn main() {
    // use lib function read() to retrieve width & height values
    // for rect1
    let mut w: u32 = lib::read("width");
    let mut h: u32 = lib::read("height");

    // Create rect1 as an instance of Rectangle with given values
    let rect1 = lib::Rectangle::new(w,h);

    // Print area of rect1 to screen
    println!("Area of rect1 = {}",rect1.area());

    // use lib function read() to retrieve width & height values
    // for rect2
    w = lib::read("width");
    h = lib::read("height");

    // Create rect2 as an instance of Rectangle with given values
    let rect2 = lib::Rectangle::new(w,h);

    // Print area of rect2 to screen
    println!("Area of rect2 = {}",rect2.area());

    // use Rectangles can_hold method to determine whether rect1
    // can hold rect2 inside of it;
    // Print the result as a string to the screen
    match rect1.can_hold(&rect2) {
        true => println!("rect2 can fit inside rect1"),
        false => println!("rect2 cannot fit inside rect1")
    }
}
