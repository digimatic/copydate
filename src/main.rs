extern crate chrono;
extern crate clipboard;

use chrono::Datelike;
use chrono::Local;
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use std::format;

fn main() {
    let now = Local::now();

    let yr = now.year();
    let mth = now.month();
    let dy = now.day();
    let date_str = format!("{yr:04}-{mth:02}-{dy:02}");


    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(date_str.to_owned()).unwrap();

    println!("{date_str}");
}
