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

struct Temperature {
    temp_type: Option<TempType>,
    value: f32,
}

fn main() {
    let temp = receive_temperature();

    match temp.temp_type {
        Some(TempType::Celcius) => convert_from_celcius(temp.value),
        Some(TempType::Fahrenheit) => convert_from_fahrenheit(temp.value),
        Some(TempType::Kelvin) => convert_from_kelvin(temp.value),
        None => {
            println!("Unrecognised type input, cannot convert temperature.");
        }
    }
}

fn receive_temperature() -> Temperature {
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

    let (temp_value, temp_type)
        = split_temperature_and_type(&temperature_and_type);

    let mut temperature_and_type = Temperature {
        temp_type: None,
        value: 0.0
    };

    temperature_and_type.value = match temp_value.trim().parse() {
        Ok(num) => num,
        Err(_) => return temperature_and_type,
    };

    // The user may have input the temperature type using the full word
    // or they may have input just the first letter of the temperature type.
    // Extract the first letter of their input.
    let temp_type = temp_type.chars().next().unwrap();

    if (temp_type == 'C') || (temp_type == 'c') {
        temperature_and_type.temp_type = Some(TempType::Celcius);
    } else if (temp_type == 'F') || (temp_type == 'f') {
        temperature_and_type.temp_type = Some(TempType::Fahrenheit);
    } else if (temp_type == 'K') || (temp_type == 'k') {
        temperature_and_type.temp_type = Some(TempType::Kelvin);
    }

    return temperature_and_type;
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

