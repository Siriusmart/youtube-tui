
use crate::structs::{MiniVideo, FullVideo};

#[derive(Debug, Clone)]
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
}
