#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "Area of the rectangle: {} square pixels.",
        area(width1, height1)
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("Area of the rectangle: {} square pixels.", rect1.area());

    println!(
        "Area of the rectangle: {} square pixels.",
        area_by_struct(&rect1)
    );

    println!("rect1 is {:?}", rect1);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_by_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
