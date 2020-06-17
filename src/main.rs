use std::io;

const WIDTH: usize = 10;
const ABSOLUTE_ZERO: f32 = -273.15;
const CELCIUS_TO_FAHRENHEIT_OFFSET: f32 = 32.0;
const CELCIUS_TO_FAHRENHEIT_SCALE: f32 = 9.0/5.0;

enum TempType {
    Celcius,
    Fahrenheit,
    Kelvin,
}

fn main() {
    let (temperature, temperature_type) = receive_temperature();

    match temperature_type {
        Some(TempType::Celcius) => convert_from_celcius(temperature),
        Some(TempType::Fahrenheit) => convert_from_fahrenheit(temperature),
        Some(TempType::Kelvin) => convert_from_kelvin(temperature),
        None => {
            println!("Unrecognised type input, cannot convert temperature.");
        }
    }
}

fn receive_temperature() -> (f32, Option<TempType>) {
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

    let (temperature, temp_type_string)
        = split_temperature_and_type(&temperature_and_type);

    let temperature: f32 = match temperature.trim().parse() {
        Ok(num) => num,
        Err(_) => return (0.0, None),
    };

    // The user may have input the temperature type using the full word
    // or they may have input just the first letter of the temperature type.
    // Extract the first letter of their input.
    let temp_type_char = temp_type_string.chars().next().unwrap();

    let mut temperature_type: Option<TempType> = None;

    if (temp_type_char == 'C') || (temp_type_char == 'c') {
        temperature_type = Some(TempType::Celcius);
    } else if (temp_type_char == 'F') || (temp_type_char == 'f') {
        temperature_type = Some(TempType::Fahrenheit);
    } else if (temp_type_char == 'K') || (temp_type_char == 'k') {
        temperature_type = Some(TempType::Kelvin);
    }

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

