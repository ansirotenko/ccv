#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use clipboard_master::{Master, ClipboardHandler, CallbackResult};
use std::io;
use arboard::{Clipboard};
use clipboard_files::{read};

#[cfg(target_os = "macos")]
mod mac;

struct Handler{
}

impl Handler {
    fn new() -> Self {
        Handler {}
    }
}

impl ClipboardHandler for Handler {
    fn on_clipboard_change(&mut self) -> CallbackResult {
        println!("Clipboard change happened!");

        // let content = self.ctx.get_contents();
        // let a = content.unwrap();

        if read().is_ok() {
            println!("was files");
        } {
            let mut clipboard = Clipboard::new().unwrap();
            if clipboard.get_image().is_ok() {
                println!("was image");
            } else {
                if clipboard.get_text().is_ok() {
                    println!("was text");
                }
            }
        }

        CallbackResult::Next
    }

    fn on_clipboard_error(&mut self, error: io::Error) -> CallbackResult {
        eprintln!("Error: {}", error);
        CallbackResult::Next
    }
}

fn main() {
    Master::new(Handler::new()).run().unwrap();
}
