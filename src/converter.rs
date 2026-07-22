use crate::history;
use colored::Colorize;

pub fn run(value: f64, from_unit: &str, to_unit: &str) {
    let from = from_unit.to_uppercase();
    let to = to_unit.to_uppercase();

    let celsius = to_celsius(value, &from);

    let result = match celsius {
        Some(c) => from_celsius(c, &to),
        None => {
            println!("{}", " Unknown source unit. Use C, F, or K.".red());
            return;
        }
    };

    match result {
        Some(converted) => {
            let label = safety_label(converted, &to);
            println!(
                "{}{} -> {}{} {}",
                value,
                unit_symbol(&from),
                format!("{:.1}", converted).bright_cyan().bold(),
                unit_symbol(&to),
                label
            );
            history::save(value, &from, converted, &to);
        }
        None => {
            println!("{}", " Unknown target unit. Use C, F, or K.".red());
        }
    }
}

fn to_celsius(value: f64, unit: &str) -> Option<f64> {
    match unit {
        "C" => Some(value),
        "F" => Some((value - 32.0) * 5.0 / 9.0),
        "K" => Some(value - 273.15),
        _ => None,
    }
}

fn from_celsius(celsius: f64, unit: &str) -> Option<f64> {
    match unit {
        "C" => Some(celsius),
        "F" => Some(celsius * 9.0 / 5.0 + 32.0),
        "K" => Some(celsius + 273.15),
        _ => None,
    }
}

fn unit_symbol(unit: &str) -> &str {
    match unit {
        "C" => "°C",
        "F" => "°F",
        "K" => "°K",
        _ => "?",
    }
}

fn safety_label(value: f64, unit: &str) -> String {
    let celsius = to_celsius(value, unit).unwrap_or(value);

    if celsius <= 4.0 {
        " Safe Zone (Refrigerated)".green().to_string()
    } else if celsius <= 8.0 {
        " Cool Zone".yellow().to_string()
    } else if celsius >= 60.0 {
        " Hot Zone (Pasteurized)".red().to_string()
    } else {
        " Ambient".normal().to_string()
    }
}
