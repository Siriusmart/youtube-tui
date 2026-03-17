#[cfg(feature = "mpv")]
use crate::global::functions::secs_display_string;
use crate::{config::*, global::structs::*};
use ratatui::{
    layout::Rect,
    style::Style,
    widgets::{Block, Borders, Paragraph},
};
use tui_additions::framework::*;

/// a message bar item, contains no fields because the message is taken from `data.global.Message`
#[derive(Clone, Copy)]
pub enum MessageBar {
    Stateless,
    /// start and end position of the seeker for mouse scrubbing
    Music {
        seeker_front: u16,
        seeker_rear: u16,
    },
    /// bar too short for seeker to display
    MusicSeekerLess,
}

impl Default for MessageBar {
    fn default() -> Self {
        Self::Stateless
    }
}

impl FrameworkItem for MessageBar {
    fn render(
        &mut self,
        frame: &mut ratatui::Frame,
        framework: &mut tui_additions::framework::FrameworkClean,
        area: ratatui::layout::Rect,
        popup_render: bool,
        _info: tui_additions::framework::ItemInfo,
    ) {
        if popup_render {
            return;
        }

        #[cfg(feature = "mpv")]
        {
            let mpv = framework.data.global.get::<MpvWrapper>().unwrap();
            if Self::is_mpv_render(framework) {
                let mut label = mpv.property("media-title".to_string()).unwrap();

                if let Some((name, ext)) = label.rsplit_once('.') {
                    if ext.len() < 5
                        && !name.len() > 13
                        && &name[name.len() - 1..name.len()] == "]"
                        && name[name.len() - 12..name.len() - 1]
                            .chars()
                            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_'))
                        && &name[name.len() - 13..name.len() - 12] == "["
                    {
                        label = name[0..name.len() - 13].to_string();
                    }
                }

                let duration = mpv
                    .property("duration".to_string())
                    .unwrap_or_default()
                    .parse::<f64>()
                    .unwrap() as u64;
                let duration_s = secs_display_string(duration as u32);
                let playerhead = mpv
                    .property("time-pos".to_string())
                    .unwrap_or_default()
                    .parse::<f64>()
                    .unwrap() as u64;
                let mut playerhead_s = secs_display_string(playerhead as u32);
                if playerhead_s.len() != duration_s.len() {
                    playerhead_s = format!(
                        "{}{playerhead_s}",
                        " ".repeat(duration_s.len() - playerhead_s.len())
                    );
                }
                let percentage = (playerhead * 100 / duration).to_string();

                let right_chunk = format!(
                    "{playerhead_s}/{duration_s} {}[{percentage}%]",
                    " ".repeat(3 - percentage.len())
                );
                let left_chunk = format!("[Now Playing]: {label}");
                let length = area.width as usize - 2;
                let total_len = right_chunk.len() + left_chunk.len();

                *framework.data.global.get_mut::<Message>().unwrap() =
                    Message::Mpv(if length > total_len + 6 {
                        let mut seeker_len = length - total_len - 4;
                        let seeker_pad = if seeker_len > 10 { seeker_len / 10 } else { 0 };
                        seeker_len -= seeker_pad * 2;
                        let seeker_pos = (seeker_len - 1) * playerhead as usize / duration as usize;

                        let seeker_front = left_chunk.len() + 1 + seeker_pad; // index of first
                                                                              // character in seeker
                        let seeker_rear = seeker_front + seeker_len - 1; // index of last character
                                                                         // in seeker
                        *self = MessageBar::Music {
                            seeker_front: seeker_front as u16,
                            seeker_rear: seeker_rear as u16,
                        };
                        format!(
                            "{left_chunk} {}├{}-{}┤{} {right_chunk}",
                            " ".repeat(seeker_pad),
                            "─".repeat(seeker_pos),
                            "─".repeat(seeker_len - seeker_pos - 1),
                            " ".repeat(seeker_pad)
                        )
                    } else if length > total_len + 3 {
                        *self = MessageBar::MusicSeekerLess;
                        format!(
                            "{left_chunk}{}{right_chunk}",
                            " ".repeat(length - total_len)
                        )
                    } else if length > 19 {
                        *self = MessageBar::MusicSeekerLess;
                        left_chunk
                    } else {
                        *self = MessageBar::MusicSeekerLess;
                        String::from("Not enough width")
                    });
            }
        }

        let message = framework.data.global.get::<Message>().unwrap();
        let appearance = framework.data.global.get::<AppearanceConfig>().unwrap();

        // the Option<TextList> that is Some if keys were captured for entering command
        let command_capture = &framework
            .data
            .global
            .get::<Status>()
            .unwrap()
            .command_capture;

        // display with different border style according to type of message and config
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(appearance.borders)
            .border_style(Style::default().fg(if command_capture.is_some() {
                appearance.colors.command_capture
            } else {
                match message {
                    Message::None => appearance.colors.outline,
                    Message::Success(_) => appearance.colors.message_success_outline,
                    Message::Error(_) => appearance.colors.message_error_outline,
                    Message::Message(_) | Message::Mpv(_) => appearance.colors.message_outline,
                }
            }));

        // if keys are captured, render the textlist instead of the message text, and exits the
        // function
        if let Some(textfield) = command_capture {
            let paragraph = Paragraph::new(":").block(block);
            frame.render_widget(paragraph, area);
            let mut textfield = textfield.clone();
            textfield.set_width(area.width - 3);
            let _ = textfield.update();
            frame.render_widget(
                textfield,
                Rect::new(area.x + 2, area.y + 1, area.width - 3, 1),
            );

            return;
        }

        let paragraph = Paragraph::new(
            message.to_string(
                &framework
                    .data
                    .global
                    .get::<MainConfig>()
                    .unwrap()
                    .message_bar_default,
            ),
        )
        .block(block);

        frame.render_widget(paragraph, area);
    }

    fn selectable(&self) -> bool {
        false
    }

    #[cfg(feature = "mpv")]
    fn mouse_passthrough(
        &mut self,
        framework: &mut FrameworkClean,
        _selected: bool,
        x: u16,
        _y: u16,
        _absolute_x: u16,
        _absolute_y: u16,
    ) -> bool {
        match self {
            Self::Music {
                seeker_front,
                seeker_rear,
            } => {
                use std::sync::mpsc;

                let seek_percentage =
                    (x as i32 - *seeker_front as i32) * 100 / (*seeker_rear - *seeker_front) as i32;
                let mpv = framework.data.global.get::<MpvWrapper>().unwrap();
                let (tx, rx) = mpsc::channel();
                mpv.sender
                    .send(MpvAction::Command {
                        name: "seek".to_string(),
                        args: vec![
                            seek_percentage.clamp(0, 100).to_string(),
                            "absolute-percent".to_string(),
                        ],
                        responder: tx,
                    })
                    .unwrap();
                if let MpvResponse::Error(e) = rx.recv().unwrap() {
                    *framework.data.global.get_mut::<Message>().unwrap() = Message::Error(e);
                }
                true
            }
            _ => false,
        }
    }
}

#[cfg(feature = "mpv")]
impl MessageBar {
    pub fn is_mpv_render(framework: &FrameworkClean) -> bool {
        framework.data.global.get::<MpvWrapper>().unwrap().playing()
            && matches!(
                framework.data.global.get::<Message>().unwrap(),
                Message::Mpv(_) | Message::None
            )
            && framework
                .data
                .global
                .get::<Status>()
                .unwrap()
                .command_capture
                .is_none()
    }
}
