//Derive is used to allow the macro println! to print the struct
#[derive(Debug)]
struct Rectangle {
    width: u32,
    heigth: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.heigth
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        heigth: 50,
    };

    println!("The rectangle is: {:#?}", &rect);

    println!(
        "The area of the rectangle is {} square pixels",
        &rect.area()
    );
}
