use std::{sync::mpsc, thread};

use libmpv::Mpv;
use typemap::Key;

pub struct MpvWrapper {
    pub sender: mpsc::Sender<MpvAction>,
    pub copier: mpsc::Receiver<MpvResponse>,
}

impl Clone for MpvWrapper {
    fn clone(&self) -> Self {
        panic!("no")
    }
}

// impl Default for MpvWrapper {
//     fn default() -> Self {
//         Self::spawn()
//     }
// }

impl MpvWrapper {
    pub fn spawn() -> Self {
        let (sender, receiver) = mpsc::channel();
        let (responder, copier) = mpsc::channel();
        thread::spawn(move || {
            let mpv = Mpv::new().unwrap();
            mpv.set_property("video", "no").unwrap();
            loop {
                let action = match receiver.recv() {
                    Ok(res) => res,
                    Err(_) => continue,
                };
                match action {
                    MpvAction::Command { name, args } => match mpv
                        .command(&name, &args.iter().map(|s| s.as_str()).collect::<Vec<_>>())
                    {
                        Ok(_) => responder.send(MpvResponse::Copy).unwrap(),
                        Err(e) => responder.send(MpvResponse::Error(e.to_string())).unwrap(),
                    },
                    MpvAction::RequestI64 { name } => responder
                        .send(MpvResponse::I64(mpv.get_property::<i64>(&name).ok()))
                        .unwrap(),
                }
            }
        });
        Self { sender, copier }
    }
}

#[derive(Debug)]
pub enum MpvAction {
    Command { name: String, args: Vec<String> },
    RequestI64 { name: String },
}

pub enum MpvResponse {
    Copy,
    I64(Option<i64>),
    Error(String),
}

impl Key for MpvWrapper {
    type Value = Self;
}
