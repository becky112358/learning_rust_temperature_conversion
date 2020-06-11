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
        println!("Converting from Celcius");
        convert_from_celcius(temperature);
    } else if temperature_type == 'F' {
        println!("Converting from Fahrenheit");
    } else if temperature_type == 'K' {
        println!("Converting from Kelvin");
    } else {
        println!("Unrecognised type input, cannot convert temperature.");
    }
}

fn convert_from_celcius(c: f32) {
    let f = (c * 1.8) + 32.0;
    let k = c + 273.15;

    println!("{} degrees Celcius", c);
    println!("{} degrees Fahrenheit", f);
    println!("{} degrees Kelvin", k);
}


