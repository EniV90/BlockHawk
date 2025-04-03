use num_format::{Locale, ToFormattedString};

pub fn format_usd(value: f64) -> String {
    let whole_part = value.trunc() as u64; // Extract whole number part
    let decimal_part = (value.fract() * 100.0).round() as u64; // Get cents

    format!(
        "${}{}.{} USD",
        whole_part.to_formatted_string(&Locale::en),
        if decimal_part < 10 { "0" } else { "" }, // Ensures two decimal places
        decimal_part
    )
}
