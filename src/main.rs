use std::io;

const WIDTH: usize = 10;

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

    if (temperature_type == 'C') || (temperature_type == 'c') {
        convert_from_celcius(temperature);
    } else if (temperature_type == 'F') || (temperature_type == 'f') {
        convert_from_fahrenheit(temperature);
    } else if (temperature_type == 'K') || (temperature_type == 'k') {
        convert_from_kelvin(temperature);
    } else {
        println!("Unrecognised type input, cannot convert temperature.");
    }
}

fn convert_from_celcius(c: f32) {
    println!("Converting from Celcius");

    let f = (c * 1.8) + 32.0;
    let k = c + 273.15;

    println!("{c:>width$} degrees Celcius", c=c, width=WIDTH);
    println!("{f:>width$} degrees Fahrenheit", f=f, width=WIDTH);
    println!("{k:>width$} degrees Kelvin", k=k, width=WIDTH);
}

fn convert_from_fahrenheit(f: f32) {
    println!("Converting from Fahrenheit");

    let c = (f - 32.0) * (5.0 / 9.0);
    let k = c + 273.15;

    println!("{c:>width$} degrees Celcius", c=c, width=WIDTH);
    println!("{f:>width$} degrees Fahrenheit", f=f, width=WIDTH);
    println!("{k:>width$} degrees Kelvin", k=k, width=WIDTH);
}

fn convert_from_kelvin(k: f32) {
    println!("Converting from Kelvin");

    let c = k - 273.15;
    let f = (c * 1.8) + 32.0;

    println!("{c:>width$} degrees Celcius", c=c, width=WIDTH);
    println!("{f:>width$} degrees Fahrenheit", f=f, width=WIDTH);
    println!("{k:>width$} degrees Kelvin", k=k, width=WIDTH);
}

