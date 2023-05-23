// Bring hellorust into scope
use hellorust;

// Initial practise for integration tests
#[test]
fn integrate_test_rectangle() {
    let rect1 = hellorust::Rectangle::new(5,5);
    let rect2 = hellorust::Rectangle::new(3,3);

    assert_eq!(25,rect1.area());
    assert_eq!(9,rect2.area());
}
