use std::io;

fn main() {
    println!("Choose mode:");
    println!("1) Celsius to Farenheit");
    println!("2) Farenheit to Celsius");

    let choice = get_input_number();

    match choice.round() as i32 {
        1 => celsius_converter(),
        2 => farenheit_converter(),
        _ => println!("did not match"),
    }
}

fn get_input_number() -> f32 {
    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    choice.trim().parse().expect("Not a number")
}

const QUOTIENT: f32 = 5.0 / 9.0;

fn celsius_converter() {
    println!("Enter the temperature you want to convert, in celsius");
    let mut temperature = get_input_number();
    temperature = temperature / QUOTIENT + 32.;
    println!("The temperature in farenheit is {} degrees", temperature)
}
fn farenheit_converter() {
    println!("Enter the temperature you want to convert, in farenheit");
    let mut temperature = get_input_number();

    temperature = (temperature - 32.0) * QUOTIENT;
    println!("The temperature in celsius is {} degrees", temperature.round())
}
