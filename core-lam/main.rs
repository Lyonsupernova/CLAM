use figlet_rs::FIGfont;
mod manager;
use crate::manager::ChatManager;

fn main() {
    // load standard font
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("Welcome to Core-LAM");
    match figure {
        Some(s) => println!("{}", s),
        None => println!("Failed to load Core-LAM font :("),
    };
    let mut chat_manager = ChatManager::new();
    chat_manager.start();
}
