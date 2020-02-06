use std::io;

pub fn run() {
    println!("Try convert celcius to farenheit and otherwise!!\nPlease enter a number");

    loop {
        let mut degree = String::new();

        io::stdin()
            .read_line(&mut degree)
            .expect("Failed to read line");

        let degree: i32 = match degree.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        fahrenheit_to_celcius(degree);
        celcius_to_fahrenheit(degree);
        break;
    }
}

fn fahrenheit_to_celcius(t: i32) {
    let to_c: i32 = t - 32;
    let _result = to_c * 5 / 9;
    println!("temperature is{:?} celcius", _result);
}

fn celcius_to_fahrenheit(t: i32) {
    let to_f: i32 = t * 9 / 5;
    let _result = to_f + 32;
    println!("temperature is {:?} fahrenheit", _result);
}
