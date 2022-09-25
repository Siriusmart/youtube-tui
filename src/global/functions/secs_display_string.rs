const SECS_IN_MINUTE: u32 = 60;
const SECS_IN_HOUR: u32 = 3600;

/// Turns seconds into pretty format like `02:43`
pub fn secs_display_string(mut secs: u32) -> String {
    let mut out = Vec::new();

    if secs >= SECS_IN_HOUR {
        let hours = secs / SECS_IN_HOUR;
        secs -= hours * SECS_IN_HOUR;
        out.push(format!("{}", hours));
    }

    if secs >= SECS_IN_MINUTE {
        let minutes = secs / SECS_IN_MINUTE;
        secs -= minutes * SECS_IN_MINUTE;
        out.push(two_digit_num(minutes));
    } else if out.len() != 0 {
        out.push(String::from("00"));
    }

    if out.len() == 0 {
        out.push(String::from("00"));
    }
    out.push(two_digit_num(secs));

    out.join(":")
}

fn two_digit_num(num: u32) -> String {
    let out = format!("{}", num);

    format!("{}{}", "0".repeat(2 - out.len()), out)
}
