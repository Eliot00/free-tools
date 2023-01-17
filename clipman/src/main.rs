mod clipboard;
mod utils;

use std::sync::mpsc;
use std::thread;

use crate::utils::ClipboardHistory;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        clipboard::handle_clipboard(tx);
    });

    let mut history: ClipboardHistory<10> = ClipboardHistory::new();
    for received in rx {
        println!("receive {:?}", received);
        history.push(received);
    }
}
