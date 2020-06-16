use std::io;

const WIDTH: usize = 10;
const ABSOLUTE_ZERO: f32 = -273.15;
const CELCIUS_TO_FAHRENHEIT_OFFSET: f32 = 32.0;
const CELCIUS_TO_FAHRENHEIT_SCALE: f32 = 9.0/5.0;

fn main() {
    let (temperature, temperature_type) = receive_temperature();

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

fn receive_temperature() -> (f32, char) {
    println!("Please state the temperature to be converted from.");
    println!("Please enter it as follows: \"temperature type\"");
    println!("Types available to convert from:");
    println!("C (Celcius)");
    println!("F (Fahrenheit)");
    println!("K (Kelvin)");

    let mut temperature_and_type = String::new();

    io::stdin()
        .read_line(&mut temperature_and_type)
        .expect("Failed to read line.");

    let (temperature, temperature_type)
        = split_temperature_and_type(&temperature_and_type);

    let temperature: f32 = match temperature.trim().parse() {
        Ok(num) => num,
        Err(_) => return (0.0, '0'),
    };

    let temperature_type: char = match temperature_type.trim().parse() {
        Ok(num) => num,
        Err(_) => return (0.0, '0'),
    };

    return (temperature, temperature_type);
}

fn split_temperature_and_type(temperature_and_type: &str) -> (&str, &str) {
    let bytes = temperature_and_type.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return (&temperature_and_type[0..i], &temperature_and_type[i+1..]);
        }
    }

    (&temperature_and_type[..], &temperature_and_type[..])
}

fn convert_from_celcius(c: f32) {
    println!("Converting from Celcius");

    let f = convert_from_celcius_to_fahrenheit(c);
    let k = convert_from_celcius_to_kelvin(c);

    print_temperatures(c, f, k);
}

fn convert_from_fahrenheit(f: f32) {
    println!("Converting from Fahrenheit");

    let c = (f - CELCIUS_TO_FAHRENHEIT_OFFSET) / CELCIUS_TO_FAHRENHEIT_SCALE;
    let k = convert_from_celcius_to_kelvin(c);

    print_temperatures(c, f, k);
}

fn convert_from_kelvin(k: f32) {
    println!("Converting from Kelvin");

    let c = k + ABSOLUTE_ZERO;
    let f = convert_from_celcius_to_fahrenheit(c);

    print_temperatures(c, f, k);
}

fn convert_from_celcius_to_fahrenheit(c: f32) -> f32 {
    (c * CELCIUS_TO_FAHRENHEIT_SCALE) + CELCIUS_TO_FAHRENHEIT_OFFSET
}

fn convert_from_celcius_to_kelvin(c: f32) -> f32 {
    c - ABSOLUTE_ZERO
}

fn print_temperatures(c: f32, f: f32, k: f32) {
    println!("{c:>width$} degrees Celcius", c=c, width=WIDTH);
    println!("{f:>width$} degrees Fahrenheit", f=f, width=WIDTH);
    println!("{k:>width$} degrees Kelvin", k=k, width=WIDTH);
}

