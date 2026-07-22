use colored::Colorize;

struct ProductProfile {
    name: &'static str,
    min_celsius: f64,
    max_celsius: f64,
    guidance: &'static str,
}

const PROFILES: &[ProductProfile] = &[
    ProductProfile {
        name: "vaccine",
        min_celsius: 2.0,
        max_celsius: 8.0,
        guidance: "Requires strict WHO cold chain standard (2°C - 8°C). Risk of loss of potency if breached.",
    },
    ProductProfile {
        name: "milk",
        min_celsius: 1.0,
        max_celsius: 4.0,
        guidance: "Dairy products spoil rapidly above 4°C due to bacterial growth.",
    },
    ProductProfile {
        name: "chocolate",
        min_celsius: 15.0,
        max_celsius: 18.0,
        guidance: "Requires controlled ambient temperature to avoid fat bloom (>18°C) or condensation (<15°C).",
    },
    ProductProfile {
        name: "ice cream",
        min_celsius: -25.0,
        max_celsius: -18.0,
        guidance: "Must remain hard-frozen below -18°C to maintain crystal structure and prevent texture loss.",
    },
    ProductProfile {
        name: "meat",
        min_celsius: -2.0,
        max_celsius: 2.0,
        guidance: "Fresh meat requires near-freezing chill storage to inhibit microbial spoilage.",
    },
    ProductProfile {
        name: "blood",
        min_celsius: 2.0,
        max_celsius: 6.0,
        guidance: "Blood bank products require strict 2°C - 6°C refrigeration to maintain red blood cell viability.",
    },
    ProductProfile {
        name: "insulin",
        min_celsius: 2.0,
        max_celsius: 8.0,
        guidance: "Unopened insulin must be kept refrigerated; never allow to freeze.",
    },
];

pub fn run(value: f64, unit: &str, product_input: &str) {
    let unit_upper = unit.to_uppercase();
    let product_normalized = product_input.trim().to_lowercase();

    let celsius = match to_celsius(value, &unit_upper) {
        Some(c) => c,
        None => {
            println!("{}", "❌ Unknown unit. Use C, F, or K.".red());
            return;
        }
    };

    let profile = PROFILES
        .iter()
        .find(|p| p.name == product_normalized || p.name.replace(' ', "-") == product_normalized);

    match profile {
        Some(p) => evaluate_safety(value, &unit_upper, celsius, p),
        None => print_unknown_product(&product_normalized),
    }
}

fn evaluate_safety(value: f64, unit: &str, celsius: f64, profile: &ProductProfile) {
    println!("\n{}", "🛡️ Cold Chain Safety Check".cyan().bold());
    println!("{}", "==========================================".cyan());
    println!("Product:     {}", profile.name.to_uppercase().bold());
    println!("Input Temp:  {:.1}°{}", value, unit);
    println!("Normalized:  {:.1}°C", celsius);
    println!(
        "Safe Range:  {:.1}°C to {:.1}°C",
        profile.min_celsius, profile.max_celsius
    );
    println!("{}", "------------------------------------------".cyan());

    if celsius < profile.min_celsius {
        println!(
            "Status:      {}",
            "❄️ CRITICAL LOW (TOO COLD / FREEZE RISK)".blue().bold()
        );
        println!("Note:        {}", profile.guidance);
    } else if celsius > profile.max_celsius {
        println!(
            "Status:      {}",
            "🔥 CRITICAL HIGH (TOO HOT / SPOILAGE RISK)".red().bold()
        );
        println!("Note:        {}", profile.guidance);
    } else {
        println!(
            "Status:      {}",
            "✅ SAFE (Within Cold Chain Limits)".green().bold()
        );
        println!("Note:        {}", profile.guidance);
    }
    println!("==========================================\n");
}

fn print_unknown_product(product: &str) {
    println!(
        "\n{} Unknown product profile: '{}'",
        "❓".yellow(),
        product.red()
    );
    println!("Available product profiles in frost-cli:");
    for p in PROFILES {
        println!(
            "  • {:<12} (Safe range: {:.1}°C to {:.1}°C)",
            p.name.cyan(),
            p.min_celsius,
            p.max_celsius
        );
    }
    println!();
}

fn to_celsius(value: f64, unit: &str) -> Option<f64> {
    match unit {
        "C" => Some(value),
        "F" => Some((value - 32.0) * 5.0 / 9.0),
        "K" => Some(value - 273.15),
        _ => None,
    }
}
