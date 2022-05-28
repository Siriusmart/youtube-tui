use tui::layout::Rect;

pub fn center_rect(rect: (u16, u16), frame_rect: Rect) -> Result<Rect, Rect> {
    let width_too_big = rect.0 > frame_rect.width;
    let height_too_big = rect.1 > frame_rect.height;

    if width_too_big || height_too_big {
        return Err(Rect {
            width: if width_too_big {
                frame_rect.width
            } else {
                rect.0
            },
            height: if height_too_big {
                frame_rect.height
            } else {
                rect.1
            },
            x: if width_too_big {
                frame_rect.x
            } else {
                (frame_rect.x + frame_rect.width - rect.0) / 2
            },
            y: if height_too_big {
                frame_rect.y
            } else {
                (frame_rect.y + frame_rect.height - rect.1) / 2
            },
        });
    }

    let x = (frame_rect.x + frame_rect.width - rect.0) / 2;
    let y = (frame_rect.y + frame_rect.height - rect.1) / 2;

    Ok(Rect::new(x, y, rect.0, rect.1))
}
