use enigo::{Enigo, KeyboardControllable};
use std::{thread, time};
extern crate clipboard;

use clipboard::{ClipboardContext, ClipboardProvider};

fn main() {
    let vreme = time::Duration::from_millis(1430);

    thread::sleep(vreme);

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let kucanje = ctx.get_contents();

    match kucanje {
        std::result::Result::Ok(i) => {
            let mut enigo = Enigo::new();
            enigo.key_sequence(&String::from(i));
        }
        Err(_) => {
            println!("not ok");
        }
    }
}
