use std::io;

fn main() {
    println!("Enter your units to convert from (f/c):");
    let mut units =  String::new();
    units = loop {
        io::stdin()
            .read_line(&mut units)
            .expect("Failed to get units");

        units = units.trim().to_string();
        if units == "f" || units == "c" {
            break units;
        } else {
            println!("Incorrect units provided enter f or c!");
        }
    };

    println!("Enter your temperature:");
    let mut temperature = String::new();
    let temperature = loop {
        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to get temperature");

        let temperature: f64 = match temperature.trim().parse() {
            Ok(temp) => temp,
            Err(_) => continue,
        };

        break temperature;
    };
    
    if units == "f" {
        println!("Originally was {temperature}D converts to {}C", convert_to_c(temperature));
    } else {
        println!("Originally was {temperature}C converts to {}F", convert_to_f(temperature));
    }
}

fn convert_to_c(f: f64) -> f64 {
    (f - 32.0) * (5.0/9.0)
}

fn convert_to_f(c: f64) -> f64 {
    (c * 1.8) + 32.0
}