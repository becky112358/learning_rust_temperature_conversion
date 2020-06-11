use std::io;

fn main() {
    println!("Please input the temperature to be converted.");

    let mut temperature = String::new();

    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line.");

    let temperature: f32 = match temperature.trim().parse() {
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

    println!("You input {} \n", temperature_type);

    if temperature_type == 'C' {
        convert_from_celcius(temperature);
    } else if temperature_type == 'F' {
        convert_from_fahrenheit(temperature);
    } else if temperature_type == 'K' {
        convert_from_kelvin(temperature);
    } else {
        println!("Unrecognised type input, cannot convert temperature.");
    }
}

fn convert_from_celcius(c: f32) {
    println!("Converting from Celcius");

    let f = (c * 1.8) + 32.0;
    let k = c + 273.15;

    println!("{} degrees Celcius", c);
    println!("{} degrees Fahrenheit", f);
    println!("{} degrees Kelvin", k);
}

fn convert_from_fahrenheit(f: f32) {
    println!("Converting from Fahrenheit");

    let c = (f - 32.0) * (5.0 / 9.0);
    let k = c + 273.15;

    println!("{} degrees Celcius", c);
    println!("{} degrees Fahrenheit", f);
    println!("{} degrees Kelvin", k);
}

fn convert_from_kelvin(k: f32) {
    println!("Converting from Kelvin");

    let c = k - 273.15;
    let f = (c * 1.8) + 32.0;

    println!("{} degrees Celcius", c);
    println!("{} degrees Fahrenheit", f);
    println!("{} degrees Kelvin", k);
}

