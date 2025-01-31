use std::io::{self, Write};

pub struct ChatManager {
    chat_history: Vec<String>,
}

impl ChatManager {
    pub fn new() -> Self {
        Self {
            chat_history: Vec::new(),
        }
    }

    pub fn start(&mut self) {
        print_help();
        loop {
            print!("> ");
            
            // flush the print buffer
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            match input {
                "/chat" => {
                    self.chat_mode();
                }
                "/clear" => {
                    self.clear_chat_history();
                }
                "/exit" => {
                    break;
                }
                "/help" => {
                print_help();
                }
                _ => {
                    println!("Invalid command. Type '/help' for a list of commands.");
                }
            }
        }
    }

    fn chat_mode(&mut self) {
        println!("Chat mode activated. Type '/exit' to exit chat mode.");
        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            if input == "/exit" {
                io::stdout().write_all("Chat mode deactivated. Type '/help' for a list of commands.\n".as_bytes()).unwrap();
                break;
            }
            io::stdout().write_all("Service is busy. Please wait.\n".as_bytes()).unwrap();
            self.chat_history.push(input.to_string());
        }
    }

    fn clear_chat_history(&mut self) {
        self.chat_history.clear();
        println!("Chat history cleared.");
    }
}

fn print_help() {
    println!("\nAvailable commands:");
    println!("/chat     - Enter chat mode (type messages until you enter '/exit')");
    println!("/clear    - Clear all chat history");
    println!("/exit     - Exit the program");
}    

