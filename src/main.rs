extern crate reqwest;
extern crate select;
extern crate winapi;

use notify_rust::Notification;
use rand::Rng;
use select::document::Document;
use select::node::Data;
use std::{thread, time};

const NO_DEF_RELEASE: &str = "Coming soon";
const STEAM_URL: &str = "https://store.steampowered.com/app/1304930/The_Outlast_Trials/";
const EMOJI_BUFFER: &[&str] = &[
    "😰", "😱", "😑", "😮", "😨", "😕", "😢", "🥺", "😭", "🤐", "😬", "😒", "😳", "😪", "😴",
];
const EMOJI_BUFFER_LEN: usize = EMOJI_BUFFER.len();

async fn get_release_date(document: Document) -> bool {
    for node in &document.nodes {
        match &node.data {
            Data::Text(text) => {
                let text_content = text.trim().to_string();

                if text_content
                    .to_lowercase()
                    .contains(&NO_DEF_RELEASE.to_lowercase())
                {
                    return false;
                }
            }

            Data::Element(_, _) => {}
            Data::Comment(_) => {}
        };
    }

    true
}

fn dispatch_notification(summary: &str, body: &str) {
    match Notification::new().summary(summary).body(body).show() {
        Ok(_) => {}
        Err(_) => {}
    }
}

#[tokio::main]
async fn main() {
    unsafe { winapi::um::wincon::FreeConsole() };

    thread::sleep(time::Duration::from_secs(60));

    loop {
        let resp = reqwest::get(STEAM_URL).await;
        match resp {
            Ok(res) => {
                let body = res.bytes().await;

                match body {
                    Ok(bod) => {
                        let document = Document::from_read(bod.as_ref());

                        match document {
                            Ok(doc) => {
                                let release = get_release_date(doc).await;

                                let release_text = if release {
                                    "Outlast Trials release date is known!".into()
                                } else {
                                    format!(
                                        "Release date not known {}",
                                        EMOJI_BUFFER
                                            [rand::thread_rng().gen_range(0, EMOJI_BUFFER_LEN)]
                                    )
                                };

                                dispatch_notification(
                                    "Outlast Trials release update!",
                                    &release_text,
                                );
                            }
                            Err(_) => dispatch_notification(
                                "Outlast-Watcher Error",
                                "Failed to read body to document",
                            ),
                        }
                    }
                    Err(_) => {
                        dispatch_notification("Outlast-Watcher Error", "Failed to fetch page body")
                    }
                }
            }
            Err(_) => {
                dispatch_notification("Outlast-Watcher Error", "Failed to request page content")
            }
        };
        thread::sleep(time::Duration::from_secs(3600));
    }
}
