use serde::*;
use std::{collections::HashMap, error::Error, sync::mpsc, thread};
// use rayon::prelude::*;
use super::{InvidiousClient, Item, MiniVideoItem};
use crate::config::MainConfig;

#[derive(Serialize, Deserialize)]
/// id, videos
pub struct Subscriptions(pub HashMap<String, Vec<MiniVideoItem>>);

impl Subscriptions {
    /// Res<(success, failed)>
    pub fn sync(&mut self, client: &InvidiousClient, mainconfig: &MainConfig) -> (u32, u32) {
        let mut failed: u32 = 0;
        let mut success: u32 = 0;

        let (tx, rx) = mpsc::channel();
        let image_index = mainconfig.image_index;

        self.0.keys().for_each(|k| {
            let tx = tx.clone();
            let client = client.clone();
            let k = k.clone();
            thread::spawn(move || {
                let res = sync_one(&k, &client, image_index).ok();
                tx.send((k, res))
            });
        });

        for res in rx {
            match res {
                (id, Some(videos)) => {
                    self.0.insert(id, videos);
                    success += 1;
                }
                (id, None) => {
                    self.0.get_mut(&id).unwrap().clear();
                    failed += 1;
                }
            }
        }

        (success, failed)
    }
}

fn sync_one(
    id: &str,
    client: &InvidiousClient,
    image_index: usize,
) -> Result<Vec<MiniVideoItem>, Box<dyn Error>> {
    Ok(client
        .0
        .channel_videos(id, None)?
        .videos
        .into_iter()
        .map(|video| {
            Item::from_channel_video(video, image_index)
                .into_minivideo()
                .unwrap()
        })
        .collect())
}
