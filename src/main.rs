use arboard::Clipboard;
use std::env;

fn main() {
    let mut clipboard = Clipboard::new().unwrap();

    let key = "GITHUBTOKEN";
    match env::var(key) {
        Ok(val) => {
            clipboard.set_text(&val).unwrap();
            println!("Token copy to clipboard");
        }
        Err(e) => println!("couldn't interpret {key}: {e}"),
    }
}
