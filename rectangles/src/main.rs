struct Rectangle {
    width: u32,
    heigth: u32,
}

fn main() {
    let rect = Rectangle {
        width: 30,
        heigth: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels",
        rectangle_area(&rect)
    )
}

fn rectangle_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.heigth
}
