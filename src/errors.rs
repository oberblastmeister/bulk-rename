use rayon::prelude::*;

pub fn combine_errors(errors: Vec<anyhow::Error>) -> String {
    format!(
        "\n{}",
        errors
            .into_par_iter()
            .map(|e| format!("{:?}", e))
            .collect::<Vec<_>>()
            .join("\n\n[bulk-rename error]:\n")
    )
}

