use tui_additions::widgets::TextField;

pub fn remove_word(field: &mut TextField) {
    if field.content[0..field.cursor].is_empty() {
        return;
    }

    let before = field.content[..field.cursor]
        .split_whitespace()
        .collect::<Vec<_>>();
    let before = if before.is_empty() {
        String::new()
    } else {
        before[0..before.len() - 1].join(" ")
    };

    field.content = format!("{}{}", before, &field.content[field.cursor..]);
    field.cursor = before.len();
}

pub fn previous_word(field: &mut TextField) {
    let bytes = field.content.as_bytes();
    while field.cursor != 0 {
        field.cursor -= 1;
        if bytes[field.cursor].is_ascii_whitespace() {
            return;
        }
    }

    field.cursor = 0;
}

pub fn next_word(field: &mut TextField) {
    let bytes = field.content.as_bytes();
    for (i, c) in bytes
        .iter()
        .enumerate()
        .take(field.content.len())
        .skip(field.cursor + 1)
    {
        if c.is_ascii_whitespace() {
            field.cursor = i;
            return;
        }
    }

    field.cursor = field.content.len();
}
