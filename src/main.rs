// Bring Rectangle struct from lib.rs into scope
use hellorust::Rectangle;
// Bring CustomSmartPointer from lib.rs into scope
use hellorust::CustomSmartPointer;
// Bring threads from std library into scope
use std::thread;

// Define main() function which will run when cargo run is called
fn main() {
    // use lib function read() to retrieve width & height values
    // for rect1
    let mut w: u32 = hellorust::read("width");
    let mut h: u32 = hellorust::read("height");

    // Create rect1 as an instance of Rectangle with given values
    let rect1 = Rectangle::new(w,h);

    // Print area of rect1 to screen
    println!("Area of rect1 = {}",rect1.area());

    // use lib function read() to retrieve width & height values
    // for rect2
    w = hellorust::read("width");
    h = hellorust::read("height");
    
    // Create rect2 as an instance of Rectangle with given values
    let rect2 = Rectangle::new(w,h);

    // Print area of rect2 to screen
    println!("Area of rect2 = {}",rect2.area());

    // use Rectangles can_hold method to determine whether rect1
    // can hold rect2 inside of it;
    // Print the result as a string to the screen
    // Perform this using a closure in a new thread
    thread::spawn(move || {
        match rect1.can_hold(&rect2) {
            true => println!("rect2 can fit inside rect1"),
            false => println!("rect2 cannot fit inside rect1")
    
        }
    });

    // Print rect2.area() again after it is passed to the new thread
    // this required Rectangle to implement Copy & Clone Traits
    println!("After thread::spawn {}",rect2.area());


    // CustomSmartPointer implements the Drop Trait
    // when CustomSmartPointer goes out of scope, the drop() trait method is called
    // its implementation for CustomSmartPointer is simply to print a message
    // stating that it is being dropped
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointers created.")
}
