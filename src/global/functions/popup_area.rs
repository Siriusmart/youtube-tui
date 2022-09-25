use tui::layout::Rect;

/// Tries to create a centered area for popup, returns `Err` if not enough space is given
pub fn popup_area(
    (max_width_percentage, max_height_percentage): (u16, u16),
    (min_width_length, min_height_length): (u16, u16),
    area: Rect,
) -> Result<Rect, Rect> {
    let mut success = true;

    let (width, x) = {
        if min_width_length >= area.width {
            success = false;
            (area.width, 0)
        } else {
            let mut width = area.width * max_width_percentage / 100;
            if width < min_width_length {
                width = min_width_length;
            }
            (width, (area.width - width) / 2)
        }
    };

    let (height, y) = {
        if min_height_length >= area.height {
            success = false;
            (area.height, 0)
        } else {
            let mut height = area.height * max_height_percentage / 100;
            if height < min_height_length {
                height = min_height_length;
            }
            (height, (area.height - height) / 2)
        }
    };

    let rect = Rect {
        x,
        y,
        width,
        height,
    };

    if success {
        Ok(rect)
    } else {
        Err(rect)
    }
}
