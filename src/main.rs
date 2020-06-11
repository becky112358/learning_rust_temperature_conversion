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
}
