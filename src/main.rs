use std::io;

fn main() {
    println!("🌡️ Temperature Converter (Fahrenheit <-> Celsius) 🌡️");

    loop {
        let (input_unit, temperature) = match get_user_input() {
            Some(values) => values,
            None => continue,
        };

        let converted_temp = if input_unit.to_lowercase() == "f" {
            fahrenheit_to_celsius(temperature)
        } else {
            celsius_to_fahrenheit(temperature)
        };

        display_result(input_unit, temperature, converted_temp);

        if !continue_conversion() {
            println!("❄️ Goodbye! ❄️");
            break;
        }
    }
}

fn get_user_input() -> Option<(String, f64)> {
    println!("\nChoose input unit (F/C):");
    let input_unit = get_valid_unit();

    println!("Enter temperature to convert:");
    let temperature = match get_valid_temperature() {
        Some(num) => num,
        None => return None,
    };

    Some((input_unit, temperature))
}

fn get_valid_unit() -> String {
    loop {
        let mut unit = String::new();
        io::stdin()
            .read_line(&mut unit)
            .expect("Failed to read line");

        let unit = unit.trim().to_lowercase();
        if unit == "f" || unit == "c" {
            return unit.to_uppercase();
        }

        println!("Invalid unit! Please enter F or C:");
    }
}

fn get_valid_temperature() -> Option<f64> {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) => return Some(num),
            Err(_) => {
                println!("Invalid number! Try again:");
                continue;
            }
        }
    }
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

fn display_result(input_unit: String, input_temp: f64, converted_temp: f64) {
    let (from_unit, to_unit) = if input_unit == "F" {
        ("°F", "°C")
    } else {
        ("°C", "°F")
    };

    println!(
        "\n{:.1}{} = {:.1}{}",
        input_temp, from_unit, converted_temp, to_unit
    );
}

fn continue_conversion() -> bool {
    loop {
        println!("\nConvert another temperature? (y/n):");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim().to_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => println!("Please enter y/n"),
        }
    }
}
