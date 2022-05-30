use crate::structs::{FullVideo, MiniVideo};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ListItem {
    MiniVideo(MiniVideo),
    FullVideo(FullVideo),
    PageTurner(bool), // true: +1 | false: -1
}

impl ListItem {
    pub fn id(&self) -> String {
        match self {
            ListItem::MiniVideo(video) => video.video_id.clone(),
            ListItem::FullVideo(video) => video.video_id.clone(),
            _ => {
                unreachable!()
            }
        }
    }

    pub fn is_page_turner(&self) -> bool {
        match self {
            ListItem::PageTurner(_) => true,
            _ => false,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            ListItem::MiniVideo(video) => video
                .title
                .chars()
                .into_iter()
                .filter(|c| !c.is_ascii_control())
                .collect(),
            ListItem::FullVideo(video) => video
                .title
                .chars()
                .into_iter()
                .filter(|c| !c.is_ascii_control())
                .collect(),
            ListItem::PageTurner(turner) => {
                if *turner {
                    String::from("Next Page")
                } else {
                    String::from("Previous Page")
                }
            }
        }
    }
}
