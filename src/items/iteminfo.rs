use tui::{
    layout::Rect,
    style::Style,
    widgets::{Paragraph, Wrap},
};
use tui_additions::framework::FrameworkItem;
#[cfg(any(feature = "sixel", feature = "halfblock"))]
use viuer::{print_from_file, Config};

use crate::{
    config::{AppearanceConfig, MainConfig},
    global::structs::{Item, Status, Task, Tasks},
};

/// an item info displays info of any `Item`s
#[derive(Clone)]
pub struct ItemInfo {
    pub item: Option<Item>,
    pub lazy_scroll: u16,
}

impl FrameworkItem for ItemInfo {
    fn render(
        &mut self,
        frame: &mut tui::Frame<tui::backend::CrosstermBackend<std::io::Stdout>>,
        framework: &mut tui_additions::framework::FrameworkClean,
        area: tui::layout::Rect,
        popup_render: bool,
        _info: tui_additions::framework::ItemInfo,
    ) {
        if popup_render {
            return;
        }

        let item = if let Some(item) = &self.item {
            item
        } else {
            return;
        };

        let main_config = framework.data.global.get::<MainConfig>().unwrap();
        let status = framework.data.global.get::<Status>().unwrap();

        // The scroll (space above) text info will be the height of the image, but if the image fail to display, the scroll will be 0
        let scroll = if !status.render_image {
            self.lazy_scroll
        } else if cfg!(any(feature = "halfblock", feature = "sixel"))
            && main_config.images.display()
            && !status.popup_opened
        {
            #[cfg(any(feature = "sixel", feature = "halfblock"))]
            let scroll;

            #[cfg(not(any(feature = "sixel", feature = "halfblock")))]
            let scroll = 0;

            #[cfg(any(feature = "sixel", feature = "halfblock"))]
            {
                let thumbnail_path = home::home_dir()
                    .unwrap()
                    .join(".cache/youtube-tui/thumbnails/")
                    .join(item.thumbnail_id());
                if thumbnail_path.exists() {
                    #[cfg(any(feature = "sixel", feature = "halfblock"))]
                    let config = Config {
                        // channel thumbnails are squares, limiting their maximum width can prevent the
                        // entire page being taken up by the image
                        width: Some(if let Item::MiniChannel(_) = item {
                            area.width / 2
                        } else if let Item::FullChannel(_) = item {
                            area.width / 4
                        } else {
                            area.width
                        } as u32),
                        x: area.x,
                        y: area.y as i16,
                        #[cfg(feature = "sixel")]
                        use_sixel: main_config.images.use_sixels(),
                        ..Default::default()
                    };

                    if let Ok((_, height)) = print_from_file(thumbnail_path, &config) {
                        scroll = height as u16;
                        framework
                            .data
                            .state
                            .get_mut::<Tasks>()
                            .unwrap()
                            .priority
                            .push(Task::LazyRendered);
                    } else {
                        scroll = self.lazy_scroll;
                    }
                } else {
                    scroll = self.lazy_scroll;
                }
            }
            scroll
        } else {
            self.lazy_scroll
        };

        self.lazy_scroll = scroll;

        let appearance = framework.data.global.get::<AppearanceConfig>().unwrap();

        // Each "span" contains a string and a Style, and they are one line max each
        // A "text" is used for descriptions in video/playlist and channels, and starts a new line if the old one runs out
        let (spans, text) = match item {
            Item::MiniVideo(minivideo) => {
                let mut out = (
                    vec![
                        (
                            String::from("[Video]"),
                            Style::default().fg(appearance.colors.item_info.tag),
                        ),
                        (
                            minivideo.title.clone(),
                            Style::default().fg(appearance.colors.item_info.title),
                        ),
                    ],
                    minivideo.description.as_ref().map(|description| {
                        (
                            description.clone(),
                            Style::default().fg(appearance.colors.item_info.description),
                        )
                    }),
                );
                if let Some(views) = &minivideo.views {
                    out.0.push((
                        format!("{} views", views),
                        Style::default().fg(appearance.colors.item_info.viewcount),
                    ));
                }
                out.0.push((
                    format!("Length: {}", minivideo.length),
                    Style::default().fg(appearance.colors.item_info.length),
                ));
                out.0.push((
                    format!("Uploaded by {}", minivideo.channel),
                    Style::default().fg(appearance.colors.item_info.author),
                ));
                if let Some(published) = &minivideo.published {
                    out.0.push((
                        format!("Published {}", published),
                        Style::default().fg(appearance.colors.item_info.published),
                    ));
                }

                out
            }
            Item::MiniPlaylist(miniplaylist) => (
                vec![
                    (
                        String::from("[Playlist]"),
                        Style::default().fg(appearance.colors.item_info.tag),
                    ),
                    (
                        miniplaylist.title.clone(),
                        Style::default().fg(appearance.colors.item_info.title),
                    ),
                    (
                        format!("Created by by {}", miniplaylist.channel),
                        Style::default().fg(appearance.colors.item_info.author),
                    ),
                    (
                        format!(
                            "{} video{}",
                            miniplaylist.video_count,
                            if miniplaylist.video_count <= 1 {
                                ""
                            } else {
                                "s"
                            }
                        ),
                        Style::default().fg(appearance.colors.item_info.video_count),
                    ),
                ],
                None,
            ),
            Item::MiniChannel(minichannel) => (
                vec![
                    (
                        String::from("[Channel]"),
                        Style::default().fg(appearance.colors.item_info.tag),
                    ),
                    (
                        minichannel.name.clone(),
                        Style::default().fg(appearance.colors.item_info.title),
                    ),
                    (
                        format!(
                            "{} subscriber{}",
                            minichannel.sub_count_text,
                            if minichannel.sub_count <= 1 { "" } else { "s" }
                        ),
                        Style::default().fg(appearance.colors.item_info.sub_count),
                    ),
                    (
                        format!(
                            "{} video{}",
                            minichannel.video_count,
                            if minichannel.video_count <= 1 {
                                ""
                            } else {
                                "s"
                            }
                        ),
                        Style::default().fg(appearance.colors.item_info.video_count),
                    ),
                ],
                Some((
                    minichannel.description.clone(),
                    Style::default().fg(appearance.colors.item_info.description),
                )),
            ),
            Item::FullVideo(fullvideo) => (
                vec![
                    (
                        String::from("[Video]"),
                        Style::default().fg(appearance.colors.item_info.tag),
                    ),
                    (
                        fullvideo.title.clone(),
                        Style::default().fg(appearance.colors.item_info.title),
                    ),
                    (
                        format!("{} views", fullvideo.views),
                        Style::default().fg(appearance.colors.item_info.viewcount),
                    ),
                    (
                        format!("{} likes", fullvideo.likes),
                        Style::default().fg(appearance.colors.item_info.likes),
                    ),
                    (
                        format!("Length: {}", fullvideo.length),
                        Style::default().fg(appearance.colors.item_info.length),
                    ),
                    (
                        format!(
                            "Uploaded by {} ({} subscribers)",
                            fullvideo.channel, fullvideo.sub_count
                        ),
                        Style::default().fg(appearance.colors.item_info.author),
                    ),
                    (
                        format!("Published {}", fullvideo.published),
                        Style::default().fg(appearance.colors.item_info.published),
                    ),
                ],
                Some((
                    fullvideo.description.clone(),
                    Style::default().fg(appearance.colors.item_info.description),
                )),
            ),
            Item::FullPlaylist(fullplaylist) => (
                vec![
                    (
                        String::from("[Playlist]"),
                        Style::default().fg(appearance.colors.item_info.tag),
                    ),
                    (
                        fullplaylist.title.clone(),
                        Style::default().fg(appearance.colors.item_info.title),
                    ),
                    (
                        format!("Created by by {}", fullplaylist.channel),
                        Style::default().fg(appearance.colors.item_info.author),
                    ),
                    (
                        format!(
                            "{} video{}",
                            fullplaylist.video_count,
                            if fullplaylist.video_count <= 1 {
                                ""
                            } else {
                                "s"
                            }
                        ),
                        Style::default().fg(appearance.colors.item_info.video_count),
                    ),
                ],
                Some((
                    fullplaylist.description.clone(),
                    Style::default().fg(appearance.colors.item_info.description),
                )),
            ),
            Item::FullChannel(fullchannel) => (
                vec![
                    (
                        if fullchannel.autogenerated {
                            String::from("[Auto generated]")
                        } else {
                            String::from("[Channel]")
                        },
                        Style::default().fg(appearance.colors.item_info.tag),
                    ),
                    (
                        fullchannel.name.clone(),
                        Style::default().fg(appearance.colors.item_info.title),
                    ),
                    (
                        format!("{} total views", fullchannel.total_views),
                        Style::default().fg(appearance.colors.item_info.viewcount),
                    ),
                    (
                        format!(
                            "{} subscriber{}",
                            fullchannel.sub_count_text,
                            if fullchannel.sub_count <= 1 { "" } else { "s" }
                        ),
                        Style::default().fg(appearance.colors.item_info.sub_count),
                    ),
                    (
                        format!("Created at {}", fullchannel.created),
                        Style::default().fg(appearance.colors.item_info.published),
                    ),
                ],
                Some((
                    fullchannel.description.clone(),
                    Style::default().fg(appearance.colors.item_info.description),
                )),
            ),
            Item::Unknown(searchitem_transitional) => (
                vec![(
                    format!("Unknown type `{}`", searchitem_transitional.r#type),
                    Style::default().fg(appearance.colors.text_error),
                )],
                Some((
                    serde_json::to_string(&searchitem_transitional).unwrap(),
                    Style::default().fg(appearance.colors.item_info.description),
                )),
            ),
            Item::Page(b) => (
                vec![(
                    if *b { "Next page" } else { "Previous page" }.to_string(),
                    Style::default().fg(appearance.colors.item_info.page_turner),
                )],
                None,
            ),
        };

        let mut y = if scroll >= area.height { 0 } else { scroll } + area.y;
        let bottom = area.bottom();

        // puts each "span" in its own line
        for (text, style) in spans.into_iter().take((bottom - y) as usize) {
            let paragraph = Paragraph::new(text).style(style);
            frame.render_widget(
                paragraph,
                Rect {
                    y,
                    height: 1,
                    ..area
                },
            );
            y += 1;
        }

        if y > bottom || text.is_none() {
            return;
        }

        // displays description only if its a non empty string
        let (text, style) = text.unwrap();
        if text.is_empty() {
            return;
        }
        let paragraph = Paragraph::new(format!("Description:\n{}", text))
            .style(style)
            .wrap(Wrap { trim: true });
        frame.render_widget(
            paragraph,
            Rect {
                y,
                height: bottom - y,
                ..area
            },
        );
    }

    fn selectable(&self) -> bool {
        true
    }
}

impl Default for ItemInfo {
    fn default() -> Self {
        Self {
            item: None,
            lazy_scroll: 0,
        }
    }
}

impl ItemInfo {
    pub fn new(item: Option<Item>) -> Self {
        Self {
            item,
            ..Default::default()
        }
    }
}
