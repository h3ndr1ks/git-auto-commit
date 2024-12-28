mod llm;

use crate::llm::{LlmChatCompletion, Message};
use llm::ollama::Ollama;
use std::io;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ollama = Ollama::new("http://127.0.0.1", 11434, "llama3.2:latest");

    // Place to store all the messages
    let mut messages: Vec<Message> = vec![];

    loop {
        // Get input from the user
        print!("\n>");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        // Process input from the user
        let input = input.trim_end();
        if input.eq_ignore_ascii_case("exit") {
            break;
        }
        messages.push(Message::user(input));

        // Generate response
        let message = ollama.chat_complete(messages.clone()).await?;
        println!("{}", message.content);
        messages.push(message);
    }

    Ok(())
}
