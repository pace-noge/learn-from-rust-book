use std::io;

fn main() {
    let conversion_type = get_conversion_type();
    // println!("Conversion type is: {:?}", conversion_type);
    let temperature = get_temperature();
    // println!("Temperature is: {}", temperature);
    if conversion_type == 1 {
        let converted_value = (f64::from(temperature) * (9.0/5.0)) + 32.0;
        println!("The temperature in fahrenheit is: {}", converted_value);
    } else {
        let converted_value = (f64::from(temperature) - 32.0) * (5.0/9.0);
        println!("The temperature in celcius is: {}", converted_value);
    }
}


fn get_conversion_type() -> usize {
    println!("[1] Celcius to fahrenheit.");
    println!("[2] Fahrenheit to celcius");

    let mut conversion_type = String::new();

    io::stdin()
        .read_line(&mut conversion_type)
        .expect("Failed to read line");
    
    let conversion_type: usize = conversion_type.trim().parse().expect("Value entered is not number.");

    if (conversion_type > 2) | (conversion_type < 1) {
        std::process::exit(0);
    } else {
        conversion_type
    }
}

fn get_temperature() -> i32 {
    println!("Input the temperature");
    let mut temperature = String::new();

    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line");
    
    let temperature: i32 = temperature.trim().parse().expect("not valid temperature");

    temperature
}