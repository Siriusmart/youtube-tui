use tui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Paragraph, Widget, Wrap},
};
use viuer::{print_from_file, Config};

use crate::structs::ListItem;

#[derive(Clone)]
pub struct ItemDisplay {
    pub item: ListItem,
}

impl Widget for ItemDisplay {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.item {
            ListItem::MiniVideo(video) => {
                let mut path = home::home_dir().expect("Cannot get your home directory");
                path.push(".cache");
                path.push("youtube-tui");
                path.push("thumbnails");
                path.push(format!("{}.png", video.video_id));
                let exists = path.exists();
                let mut image_transform = area;
                if exists {
                    if image_transform.width * 9 > image_transform.height * 2 * 16 {
                        image_transform.width = image_transform.height * 2 * 16 / 9;
                    } else {
                        image_transform.height = image_transform.width * 9 / 32;
                    }

                    let conf = Config {
                        width: Some(image_transform.width as u32),
                        height: Some(image_transform.height as u32),
                        x: image_transform.x,
                        y: image_transform.y as i16,
                        ..Default::default()
                    };

                    let _ = print_from_file(path.as_os_str(), &conf);
                }

                let mut y = if exists {
                    image_transform.y + image_transform.height
                } else {
                    area.y
                };

                let mut to_print = Vec::new();
                to_print.push((String::from("[Video]"), Style::default().fg(Color::Gray)));
                to_print.push((video.title, Style::default().fg(Color::LightBlue)));
                to_print.push((
                    format!("Uploaded by {}", video.author),
                    Style::default().fg(Color::LightGreen),
                ));
                if let Some(s) = video.view_count {
                    to_print.push((
                        format!("Views: {}", s),
                        Style::default().fg(Color::LightYellow),
                    ));
                }
                to_print.push((
                    format!("Length: {}", video.length),
                    Style::default().fg(Color::LightCyan),
                ));
                if let Some(s) = video.published {
                    to_print.push((
                        format!("Published: {}", s),
                        Style::default().fg(Color::LightMagenta),
                    ));
                }

                for (mut item, style) in to_print {
                    if y > area.height + area.y - 1 {
                        return;
                    }

                    if item.len() > area.width as usize {
                        if item.len() > area.width as usize - 2 {
                            for _ in area.width as usize - 3..item.len() {
                                item.pop();
                            }

                            item.push('…');
                        }
                    }
                    buf.set_string(area.x, y, item, style);
                    y += 1;
                }
            }

            ListItem::FullVideo(video) => {
                let mut path = home::home_dir().expect("Cannot get your home directory");
                path.push(".cache");
                path.push("youtube-tui");
                path.push("thumbnails");
                path.push(format!("{}.png", video.video_id));
                let exists = path.exists();
                let mut image_transform = area;
                if exists {
                    if image_transform.width * 9 > image_transform.height * 2 * 16 {
                        image_transform.width = image_transform.height * 2 * 16 / 9;
                    } else {
                        image_transform.height = image_transform.width * 9 / 32;
                    }

                    let conf = Config {
                        width: Some(image_transform.width as u32),
                        height: Some(image_transform.height as u32),
                        x: image_transform.x,
                        y: image_transform.y as i16,
                        ..Default::default()
                    };

                    let _ = print_from_file(path.as_os_str(), &conf);
                }

                let mut y = if exists {
                    image_transform.y + image_transform.height
                } else {
                    area.y
                };

                let mut to_print = Vec::new();
                let mut tags = Vec::new();

                if !video.nsfw {
                    tags.push("Family Friendly");
                }

                if !video.public {
                    tags.push("Unlisted")
                }

                if video.upcoming {
                    tags.push("Upcoming")
                }
                if video.live {
                    tags.push("Live")
                }

                tags.push("Video");

                to_print.push((
                    format!(" [{}]", tags.join(" ")),
                    Style::default().fg(Color::Gray),
                ));
                to_print.push((video.title, Style::default().fg(Color::LightBlue)));
                to_print.push((
                    format!("Uploaded by {}", video.channel),
                    Style::default().fg(Color::LightGreen),
                ));
                to_print.push((
                    format!("Views: {}", video.views),
                    Style::default().fg(Color::LightYellow),
                ));
                to_print.push((
                    format!("Length: {}", video.length),
                    Style::default().fg(Color::LightCyan),
                ));
                to_print.push((
                    format!("Genre: {}", video.genre),
                    Style::default().fg(Color::LightYellow),
                ));
                to_print.push((
                    format!("Published {}", video.published),
                    Style::default().fg(Color::LightMagenta),
                ));

                to_print.push((
                    String::from("Description: "),
                    Style::default().fg(Color::Gray),
                ));

                for (mut item, style) in to_print {
                    if y > area.height + area.y - 1 {
                        return;
                    }

                    if item.len() > area.width as usize {
                        item = format!("{}...", &item[..area.width as usize - 3]);
                    }
                    buf.set_string(area.x, y, item, style);
                    y += 1;
                }

                if y < area.height + area.y {
                    let rect = Rect {
                        x: area.x,
                        y: y,
                        width: area.width,
                        height: area.height + area.y - y,
                    };

                    let paragraph = Paragraph::new(video.description.clone())
                        .wrap(Wrap { trim: true })
                        .alignment(Alignment::Left)
                        .style(Style::default().fg(Color::Gray));

                    paragraph.render(rect, buf);
                }
            }

            ListItem::MiniPlayList(playlist) => {
                let mut path = home::home_dir().expect("Cannot get your home directory");
                path.push(".cache");
                path.push("youtube-tui");
                path.push("thumbnails");
                path.push(format!("{}.png", playlist.playlist_id));
                let exists = path.exists();
                let mut image_transform = area;
                if exists {
                    if image_transform.width * 9 > image_transform.height * 2 * 16 {
                        image_transform.width = image_transform.height * 2 * 16 / 9;
                    } else {
                        image_transform.height = image_transform.width * 9 / 32;
                    }

                    let conf = Config {
                        width: Some(image_transform.width as u32),
                        height: Some(image_transform.height as u32),
                        x: image_transform.x,
                        y: image_transform.y as i16,
                        ..Default::default()
                    };

                    let _ = print_from_file(path.as_os_str(), &conf);
                }

                let mut y = if exists {
                    image_transform.y + image_transform.height
                } else {
                    area.y
                };

                let mut to_print = Vec::new();
                to_print.push((String::from("[PlayList]"), Style::default().fg(Color::Gray)));
                to_print.push((playlist.title, Style::default().fg(Color::LightBlue)));
                to_print.push((
                    format!("Created by {}", playlist.author),
                    Style::default().fg(Color::LightGreen),
                ));
                to_print.push((
                    format!("Video Count: {}", playlist.video_count),
                    Style::default().fg(Color::LightCyan),
                ));

                for (mut item, style) in to_print {
                    if y > area.height + area.y - 1 {
                        return;
                    }

                    if item.len() > area.width as usize {
                        if item.len() > area.width as usize - 2 {
                            for _ in area.width as usize - 3..item.len() {
                                item.pop();
                            }

                            item.push('…');
                        }
                    }
                    buf.set_string(area.x, y, item, style);
                    y += 1;
                }
            }

            ListItem::FullPlayList(playlist) => {
                let mut path = home::home_dir().expect("Cannot get your home directory");
                path.push(".cache");
                path.push("youtube-tui");
                path.push("thumbnails");
                path.push(format!("{}.png", playlist.playlist_id));
                let exists = path.exists();
                let mut image_transform = area;
                if exists {
                    if image_transform.width * 9 > image_transform.height * 2 * 16 {
                        image_transform.width = image_transform.height * 2 * 16 / 9;
                    } else {
                        image_transform.height = image_transform.width * 9 / 32;
                    }

                    let conf = Config {
                        width: Some(image_transform.width as u32),
                        height: Some(image_transform.height as u32),
                        x: image_transform.x,
                        y: image_transform.y as i16,
                        ..Default::default()
                    };

                    let _ = print_from_file(path.as_os_str(), &conf);
                }

                let mut y = if exists {
                    image_transform.y + image_transform.height
                } else {
                    area.y
                };

                let mut to_print = Vec::new();
                to_print.push((String::from("[PlayList]"), Style::default().fg(Color::Gray)));
                to_print.push((playlist.title, Style::default().fg(Color::LightBlue)));
                to_print.push((
                    format!("Created by {}", playlist.author),
                    Style::default().fg(Color::LightGreen),
                ));
                to_print.push((
                    format!("Video Count: {}", playlist.video_count),
                    Style::default().fg(Color::LightCyan),
                ));
                to_print.push((
                    format!("Views: {}", playlist.views),
                    Style::default().fg(Color::LightYellow),
                ));

                for (mut item, style) in to_print {
                    if y > area.height + area.y - 1 {
                        return;
                    }
                    
                    if item.len() > area.width as usize {
                        if item.len() > area.width as usize - 2 {
                            for _ in area.width as usize - 3..item.len() {
                                item.pop();
                            }

                            item.push('…');
                        }
                    }

                    buf.set_string(area.x, y, item, style);

                    y += 1;
                }
            }

            ListItem::PageTurner(b) => {
                let paragraph = Paragraph::new(if b {
                    "Press enter to go to the next page"
                } else {
                    "Press enter to go to the previous page"
                })
                .style(Style::default().bg(Color::DarkGray))
                .wrap(Wrap { trim: true })
                .alignment(Alignment::Center);

                paragraph.render(area, buf);
            }
            
        }
    }
}
