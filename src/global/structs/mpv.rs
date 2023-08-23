use std::{sync::mpsc, thread};

use libmpv::Mpv;
use typemap::Key;

pub struct MpvWrapper {
    pub sender: mpsc::Sender<MpvAction>,
    // pub copier: mpsc::Receiver<MpvResponse>,
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
        // let (responder, copier) = mpsc::channel();
        thread::spawn(move || {
            let mpv = Mpv::new().unwrap();
            mpv.set_property("video", "no").unwrap();
            loop {
                let action = match receiver.recv() {
                    Ok(res) => res,
                    Err(_) => continue,
                };
                match action {
                    MpvAction::Command {
                        name,
                        args,
                        responder,
                    } => match mpv
                        .command(&name, &args.iter().map(|s| s.as_str()).collect::<Vec<_>>())
                    {
                        Ok(_) => responder.send(MpvResponse::Copy).unwrap(),
                        Err(e) => responder.send(MpvResponse::Error(e.to_string())).unwrap(),
                    },
                    MpvAction::GetProperty { name, responder } => responder
                        .send(MpvResponse::Property(
                            mpv.get_property::<String>(&name).ok(),
                        ))
                        .unwrap(),
                    MpvAction::SetProperty {
                        name,
                        value,
                        responder,
                    } => responder
                        .send(match mpv.set_property(&name, value) {
                            Ok(_) => MpvResponse::Copy,
                            Err(e) => MpvResponse::Error(e.to_string()),
                        })
                        .unwrap(),
                }
            }
        });
        Self { sender }
    }

    pub fn command(&self, name: String, args: Vec<String>) -> MpvResponse {
        let (tx, rx) = mpsc::channel();
        self.sender
            .send(MpvAction::Command {
                name,
                args,
                responder: tx,
            })
            .unwrap();

        rx.recv().unwrap()
    }

    pub fn property(&self, prop: String) -> Option<String> {
        let (tx, rx) = mpsc::channel();
        self.sender
            .send(MpvAction::GetProperty {
                name: prop,
                responder: tx,
            })
            .unwrap();

        if let MpvResponse::Property(p) = rx.recv().unwrap() {
            p
        } else {
            unreachable!()
        }
    }

    pub fn set_property(&self, name: String, value: String) -> MpvResponse {
        let (tx, rx) = mpsc::channel();
        self.sender
            .send(MpvAction::SetProperty {
                name,
                value,
                responder: tx,
            })
            .unwrap();

        rx.recv().unwrap()
    }
}

impl MpvWrapper {
    pub fn playing(&self) -> bool {
        self.property("core-idle".to_string())
            .is_some_and(|s| s.as_str() == "no" || s.as_str() == "false")
    }
}

#[derive(Debug)]
pub enum MpvAction {
    Command {
        name: String,
        args: Vec<String>,
        responder: mpsc::Sender<MpvResponse>,
    },
    GetProperty {
        name: String,
        responder: mpsc::Sender<MpvResponse>,
    },
    SetProperty {
        name: String,
        value: String,
        responder: mpsc::Sender<MpvResponse>,
    },
}

pub enum MpvResponse {
    Copy,
    Property(Option<String>),
    Error(String),
}

impl Key for MpvWrapper {
    type Value = Self;
}
