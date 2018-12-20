use std::io;

fn main() {
    println!("Hello, welcome to the temperature conversor!");

    loop {
        println!("Please, tell what do you want to convert: ");
        println!("1: Celcius to Fahrenheit");
        println!("2: Fahrenheit to Celcius ");

        let mut conversor = String::new();

        io::stdin()
            .read_line(&mut conversor)
            .expect("Failed to read line");

        let conversor: u8 = match conversor.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("----------------------------------------");

        println!("Please, tell me the temperature to convert: ");

        let mut temperature = String::new();

        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line");

        let temperature: u32 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("----------------------------------------");

        if conversor == 1 {
            println!("{} celsius degrees converted to fahrenheit is: {}", temperature, to_fahrenheit(temperature))
        } else {
            println!("{} fahrenheit degrees converted to celsius is: {}", temperature, to_celsius(temperature))
        }
        break;
    }
}

fn to_fahrenheit (degrees: u32) -> f32 {
    let formula = ((degrees * 9) / 5) + 32;
    let converted = formula as f32;

    return converted;
}

fn to_celsius (degrees: u32) -> f32 {
    let formula = ((degrees - 32) * 5) / 9;
    let converted = formula as f32;

    return converted;
}
