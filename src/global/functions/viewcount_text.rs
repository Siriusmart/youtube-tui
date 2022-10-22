const UNITS: &[(&str, f64)] = &[
    ("B", 1_000_000_000_f64),
    ("M", 1_000_000_f64),
    ("K", 1_000_f64),
    ("", 1_f64),
];

/// Turns number into short hand like `1.56M` instead of `1560000`
pub fn viewcount_text(views: u64) -> String {
    for unit in UNITS.iter() {
        if unit.1 == 1_f64 {
            return format!("{}", views);
        } else if views >= unit.1 as u64 {
            return format!("{:.2}{}", views as f64 / unit.1, unit.0);
        }
    }

    String::from("No views")
}
