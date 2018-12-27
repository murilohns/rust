//Derive is used to allow the macro println! to print the struct
#[derive(Debug)]
struct Rectangle {
    width: u32,
    heigth: u32,
}

fn main() {
    let rect = Rectangle {
        width: 30,
        heigth: 50,
    };

    println!("The rectangle is: {:?}", &rect);

    println!(
        "The area of the rectangle is {} square pixels",
        rectangle_area(&rect)
    );
}

fn rectangle_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.heigth
}
