use std::{fs::{self, File}, thread, time::Duration, process::Command, io::Write};

use chrono::Datelike;
use home::home_dir;

use crate::global::functions::fake_rand_range;

pub fn egg() {
    let date_chrono = chrono::Local::now().date_naive();

    let month = date_chrono.month();
    let date = date_chrono.day();

    let home_dir = home_dir().unwrap();
    let displayed_store_path = home_dir.join(".local/share/youtube-tui/eggs/");
    let video_cache_path = home_dir.join(".cache/youtube-tui/eggs/");

    if !displayed_store_path.exists() {
        let _ = fs::create_dir_all(displayed_store_path);
    }

    if !video_cache_path.exists() {
        let _ = fs::create_dir_all(video_cache_path);
    }

    match (month, date) {
        (12, 25) | (12, 26) => if !displayed("christmas") {
            thread::spawn(|| {
                thread::sleep(Duration::from_secs(fake_rand_range(30_i64, 250) as u64));

                store("christmas", "happy christmas ;)");
                let _ = Command::new("mpv").args(["--no-terminal", CHRISTMAS_VIDEOS[fake_rand_range(0_i64, CHRISTMAS_VIDEOS.len() as i64) as usize]]).output();
            });
        },
        _ => {}
    }
}

fn displayed(label: &str) -> bool {
    let home_dir = home_dir().unwrap();
    let year = chrono::Local::now().date_naive().year();
    let file_path = home_dir.join(".local/share/youtube-tui/eggs/").join(format!("{label}{year}"));

    file_path.exists()
}

fn store(label: &str, content: &str) {
    let home_dir = home_dir().unwrap();
    let year = chrono::Local::now().date_naive().year();
    let file_path = home_dir.join(".local/share/youtube-tui/eggs/").join(format!("{label}{year}"));


    let mut file = File::create(file_path).unwrap();
    let _ =file.write_all(content.as_bytes());
}

const CHRISTMAS_VIDEOS: &[&str] = &["https://youtu.be/YH7LouNgIvE", "https://youtu.be/KsInh3VvE3s"];
