const UNITS: &[(&'static str, f64)] = &[
    ("B", 1_000_000_000_f64),
    ("M", 1_000_000_f64),
    ("K", 1_000_f64),
    ("", 1_f64),
];

// Turns number into short hand like 1.56M instead of its full form
pub fn viewcount_text(views: u64) -> String {
    for unit in UNITS.iter() {
        if views >= unit.1 as u64 {
            return format!("{:.2}{}", views as f64 / unit.1, unit.0);
        }
    }

    unreachable!()
}
