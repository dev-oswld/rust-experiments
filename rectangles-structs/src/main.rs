//Debugging information
#[derive(Debug)]

//A piece of data being grouped together,
//like a object data
struct Rectangle {
    width: u32,
    height: u32,
}

//Method syntax
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

//Associated Function
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };
    let sq = Rectangle::square(3); //Extra
    
    //Debug println!("The rectangle one is {:?}", rect1);
    println!("The area of the rectangle is {} square pixels.", rect1.area());
    println!("Can rectangle one hold rectangle two? {}", rect1.can_hold(&rect2));
    println!("Can rectangle one hold rectangle three? {}", rect1.can_hold(&rect3));
    println!("The square is {:?}.", sq);
}