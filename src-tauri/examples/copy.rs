#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use clipboard_master::{Master, ClipboardHandler, CallbackResult};

use std::io;
use arboard::{Clipboard};
use clipboard_files::{read};
use objc2::{
	msg_send_id,
	rc::{autoreleasepool, Id},
	runtime::ProtocolObject,
	ClassType,
};
use objc2_app_kit::{NSPasteboard, NSPasteboardTypeHTML, NSPasteboardTypeString};
use objc2_foundation::{NSArray, NSString, NSURL};
use std::{
	borrow::Cow,
	panic::{RefUnwindSafe, UnwindSafe},
};

pub fn paste(s: &str) {
    let pasteboardOtp: Option<Id<NSPasteboard>> =
			unsafe { msg_send_id![NSPasteboard::class(), generalPasteboard] };

    let pasteboard: Id<NSPasteboard> = pasteboardOtp.unwrap();

    unsafe { pasteboard.clearContents() };
	let nsurl = unsafe { NSURL::fileURLWithPath(&NSString::from_str(s)) };
    let url_array = NSArray::from_vec(vec![ProtocolObject::from_id(nsurl)]);
	unsafe { pasteboard.writeObjects(&url_array) };
}

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
        // /Users/margaritaloginova/dev/ccv/README.md

        if read().is_ok() {
            println!("was files");
        } else {
            let mut clipboard = Clipboard::new().unwrap();
            if clipboard.get_image().is_ok() {
                println!("was image");
            } else {
                let text = clipboard.get_text();
                if let Ok(textData) = text {
                    println!("was text");
                    paste(&textData);
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
