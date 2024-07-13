use crate::common::Error;
use objc2::{
	msg_send_id,
	rc::{autoreleasepool, Id},
	runtime::ProtocolObject,
	ClassType,
};
use objc2_app_kit::{NSPasteboard, NSPasteboardTypeHTML, NSPasteboardTypeString, NSURL};
use objc2_foundation::{NSArray, NSString};
use std::{
	borrow::Cow,
	panic::{RefUnwindSafe, UnwindSafe},
};

pub fn paste(s: &str) {
    let pasteboardOtp: Option<Id<NSPasteboard>> =
			unsafe { msg_send_id![NSPasteboard::class(), generalPasteboard] };

    let pasteboard: Id<NSPasteboard> = pasteboardOtp.unwrap();

    unsafe { pasteboard.clearContents() };

    let url_array = NSArray::from_vec(vec![ProtocolObject::from_id(NSURL::absolute_string(s))]);
	unsafe { self.clipboard.pasteboard.writeObjects(&url_array) };
}