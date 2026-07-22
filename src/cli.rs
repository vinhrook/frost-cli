use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "frost-cli")]
#[command(about = "❄️ A cold chain temperature converter and safety checker")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Convert a Temperature between units (e.g. `convert 23 C to F` or `convert 23 --unit C --to F`)
    Convert {
        /// Temperature value to convert
        value: f64,
        /// Source unit: C, F, or K
        #[arg(long)]
        unit: Option<String>,
        /// Target unit: C, F, or K
        #[arg(long)]
        to: Option<String>,
        /// Positional arguments for units (e.g. "C to F" or "C F")
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },
    /// Check if temperature is safe for a product (e.g. `check 4 C vaccine` or `check 4 vaccine`)
    Check {
        /// Temperature value to check
        value: f64,
        /// Unit: C, F, or K
        #[arg(long)]
        unit: Option<String>,
        /// Product name (e.g. vaccine, milk, chocolate)
        #[arg(long)]
        product: Option<String>,
        /// Positional arguments (e.g. "C vaccine" or "vaccine")
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },
    History,
}
