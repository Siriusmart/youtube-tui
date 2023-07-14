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
