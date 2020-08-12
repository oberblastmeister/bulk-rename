use rayon::prelude::*;
use colored::Colorize;

pub fn print_error(msg: impl Into<String>) {
    eprintln!("{}: {}", "Error".red().bold(), msg.into());
}

pub fn anyhow_multiple(errors: Vec<anyhow::Error>) -> String {
    format!(
        "\n{}",
        errors
            .into_par_iter()
            .map(|e| format!("{:?}", e))
            .collect::<Vec<_>>()
            .join(&format!("\n\n{}:\n", "Error".red().bold()))
    )
}

