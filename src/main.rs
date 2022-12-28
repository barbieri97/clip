use arboard::Clipboard;
// use std::env;
use std::io;

fn main() {
    let mut clipboard = Clipboard::new().unwrap();

    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer){
        Ok(n) => {
            clipboard.set_text(&*buffer.trim()).unwrap();
            println!("{n} bytes copy to clipboard");
        }
        Err(error) => println!("error: {error}"),
    }

    // Implementar a possibilidade de passar uma variavel de ambiente
    // como argumento para add no clipboard
    // TODO
    // let key = "GITHUBTOKEN";
    // match env::var(key) {
    //     Ok(val) => {
    //         clipboard.set_text(&val).unwrap();
    //         println!("Token copy to clipboard");
    //     }
    //     Err(e) => println!("couldn't interpret {key}: {e}"),
    // }

}
