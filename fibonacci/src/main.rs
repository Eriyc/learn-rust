use std::io;

fn main() {
    println!("Enter the n:th fibonacci number you want");
    let iterations = get_input_number().round() as u32;

    let mut fibonacci = (0, 1);

    if iterations == 1 {
        println!("The {} fibonacci number is:", iterations);
        println!("{}", fibonacci.0)
    } else {
        println!("The {}:th fibonacci sequence is:", iterations);
        for _ in 0..iterations {
            println!("{}", fibonacci.0);
            fibonacci = (fibonacci.1, fibonacci.0 + fibonacci.1);
        }
    }

    println!("The {} fibonacci number is: {}", iterations, fibonacci.0);
}

fn get_input_number() -> f32 {
    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    choice.trim().parse().expect("Not a number")
}
