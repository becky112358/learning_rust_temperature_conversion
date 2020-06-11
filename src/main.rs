use std::io;

fn main() {
    println!("Please input the temperature to be converted.");

    let mut temperature = String::new();

    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line.");

    let temperature: i32 = match temperature.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    println!("You input {}", temperature);

    println!("Please state the temperature type to be converted from.");
    println!("Options:");
    println!("C (Celsius)");
    println!("F (Fahrenheit)");
    println!("K (Kelvin)");

    let mut temperature_type = String::new();

    io::stdin()
        .read_line(&mut temperature_type)
        .expect("Failed to read line.");

    let temperature_type: char = match temperature_type.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    println!("You input {}", temperature_type);
}
