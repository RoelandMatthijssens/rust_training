#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        self.width > rectangle.width && self.height > rectangle.height
    }
}

fn main() {
    let rect1 = Rectangle {width: 30, height: 50};
    let rect2 = Rectangle {width: 10, height: 40};
    let rect3 = Rectangle {width: 60, height: 45};
    println!("Can {:?} hold {:?}?: {}", rect1, rect2, rect1.can_hold(&rect2));
    println!("Can {:?} hold {:?}?: {}", rect1, rect3, rect1.can_hold(&rect3));
    println!("Can {:?} hold {:?}?: {}", rect2, rect1, rect2.can_hold(&rect1));
    println!("Can {:?} hold {:?}?: {}", rect2, rect3, rect2.can_hold(&rect3));
    println!("Can {:?} hold {:?}?: {}", rect3, rect1, rect3.can_hold(&rect1));
    println!("Can {:?} hold {:?}?: {}", rect3, rect2, rect3.can_hold(&rect2));
}
