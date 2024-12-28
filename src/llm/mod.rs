pub mod ollama;

#[derive(Clone)]
pub struct Message {
    // e.g. valid Ollama values seem to be: User, Assistant, System, Tool
    pub role: String,    // Stored as string, to make this as modular as possible.
    pub content: String, // The message content.
}

impl Message {
    pub fn user(content: impl ToString) -> Self {
        Message {
            role: "User".to_string(),
            content: content.to_string(),
        }
    }

    pub fn assistant(content: impl ToString) -> Self {
        Message {
            role: "Assistant".to_string(),
            content: content.to_string(),
        }
    }
}

pub trait LlmChatCompletion {
    async fn chat_complete(&self, messages: Vec<Message>) -> Result<Message, anyhow::Error>;
}

pub trait LlmCompletion {
    async fn complete(&self, prompt: impl ToString) -> Result<String, anyhow::Error>;
}
