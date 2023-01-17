use std::sync::mpsc;
use std::{thread, time};

use objc2::foundation::{NSArray, NSDictionary, NSObject, NSString};
use objc2::rc::{Id, Shared};
use objc2::runtime::{Class, Object};
use objc2::{class, msg_send};

#[link(name = "AppKit", kind = "framework")]
extern "C" {}

pub fn handle_clipboard(tx: mpsc::Sender<String>) {
    let cls = Class::get("NSPasteboard").unwrap();

    let pasteboard: *mut Object = unsafe { msg_send![cls, generalPasteboard] };

    let mut cur_count: isize = unsafe { msg_send![pasteboard, changeCount] };

    loop {
        let count: isize = unsafe { msg_send![pasteboard, changeCount] };

        if cur_count == count {
            thread::sleep(time::Duration::from_millis(500));
            continue;
        }

        let string_class = class!(NSString);

        let classes: Id<NSArray<NSObject, Shared>, Shared> = unsafe {
            let array = msg_send![class!(NSArray), arrayWithObject: string_class];
            Id::new(array).unwrap()
        };

        let options: Id<NSDictionary<NSObject, NSObject>, Shared> = NSDictionary::new();
        let string_array: Id<NSArray<NSString, Shared>, Shared> = unsafe {
            let obj: *mut NSArray<NSString, Shared> =
                msg_send![pasteboard, readObjectsForClasses:&*classes options:&*options];
            Id::new(obj).unwrap()
        };

        if string_array.len() > 0 {
            let content = string_array[0].to_string();
            tx.send(content).unwrap();
        }

        cur_count = count;
    }
}
