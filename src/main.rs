mod checker;
mod cli;
mod converter;
mod history;

use clap::Parser;
use cli::{Cli, Commands};
use colored::Colorize;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Convert {
            value,
            unit,
            to,
            args,
        } => {
            let (from_unit, to_unit) = resolve_convert_args(unit, to, args);
            match (from_unit, to_unit) {
                (Some(from), Some(to_u)) => converter::run(value, &from, &to_u),
                _ => {
                    println!("{}", "❌ Please specify source and target units.".red());
                    println!("Usage examples:");
                    println!("  frost-cli convert 23 C to F");
                    println!("  frost-cli convert 23 C F");
                    println!("  frost-cli convert 23 --unit C --to F");
                }
            }
        }
        Commands::Check {
            value,
            unit,
            product,
            args,
        } => {
            let (unit_res, product_res) = resolve_check_args(unit, product, args);
            match (unit_res, product_res) {
                (Some(u), Some(p)) => checker::run(value, &u, &p),
                _ => {
                    println!("{}", "❌ Please specify a product to check.".red());
                    println!("Usage examples:");
                    println!("  frost-cli check 4 C vaccine");
                    println!("  frost-cli check 4 vaccine");
                    println!("  frost-cli check 4 --unit C --product vaccine");
                }
            }
        }
        Commands::History => {
            history::show();
        }
    }
}

fn resolve_convert_args(
    unit: Option<String>,
    to: Option<String>,
    args: Vec<String>,
) -> (Option<String>, Option<String>) {
    let mut from_res = unit;
    let mut to_res = to;

    let filtered_args: Vec<String> = args
        .into_iter()
        .filter(|a| a.to_lowercase() != "to")
        .collect();

    if from_res.is_none() && !filtered_args.is_empty() {
        from_res = Some(filtered_args[0].clone());
    }

    if to_res.is_none() {
        if from_res.is_some() && filtered_args.len() >= 2 {
            to_res = Some(filtered_args[1].clone());
        } else if from_res.is_some() && filtered_args.len() == 1 && from_res.as_ref() != Some(&filtered_args[0]) {
            to_res = Some(filtered_args[0].clone());
        }
    }

    (from_res, to_res)
}

fn resolve_check_args(
    unit: Option<String>,
    product: Option<String>,
    args: Vec<String>,
) -> (Option<String>, Option<String>) {
    let mut unit_res = unit;
    let mut product_res = product;

    if args.len() >= 2 {
        if unit_res.is_none() {
            unit_res = Some(args[0].clone());
        }
        if product_res.is_none() {
            product_res = Some(args[1].clone());
        }
    } else if args.len() == 1 {
        let arg = &args[0];
        let arg_lower = arg.to_lowercase();
        if is_unit(&arg_lower) {
            if unit_res.is_none() {
                unit_res = Some(arg.clone());
            }
        } else {
            if product_res.is_none() {
                product_res = Some(arg.clone());
            }
            if unit_res.is_none() {
                unit_res = Some("C".to_string());
            }
        }
    } else if args.is_empty() {
        if unit_res.is_none() && product_res.is_some() {
            unit_res = Some("C".to_string());
        }
    }

    (unit_res, product_res)
}

fn is_unit(s: &str) -> bool {
    matches!(s, "c" | "f" | "k" | "celsius" | "fahrenheit" | "kelvin")
}
