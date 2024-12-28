use crate::llm::{LlmChatCompletion, LlmCompletion, Message};
use anyhow::{bail, Error};
use ollama_rs::generation::chat::request::ChatMessageRequest;
use ollama_rs::generation::chat::{ChatMessage, MessageRole};
use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::IntoUrl;

impl TryInto<ChatMessage> for Message {
    type Error = Error;

    fn try_into(self) -> Result<ChatMessage, Self::Error> {
        let role = match self.role.as_str() {
            "User" => MessageRole::User,
            "Assistant" => MessageRole::Assistant,
            "System" => MessageRole::System,
            "Tool" => MessageRole::Tool,
            _ => bail!("Invalid role. Can not be converted to valid ollama message role."),
        };

        Ok(ChatMessage {
            role,
            content: self.content,
            images: None,
        })
    }
}

impl Into<Message> for ChatMessage {
    fn into(self) -> Message {
        let role_str = match self.role {
            MessageRole::User => "User",
            MessageRole::Assistant => "Assistant",
            MessageRole::System => "System",
            MessageRole::Tool => "Tool",
        };

        Message {
            role: role_str.to_string(),
            content: self.content,
        }
    }
}

pub struct Ollama {
    ollama: ollama_rs::Ollama,
    model: String,
}

impl Ollama {
    pub fn new(host: impl IntoUrl, port: u16, model: impl ToString) -> Self {
        Ollama {
            ollama: ollama_rs::Ollama::new(host, port),
            model: model.to_string(),
        }
    }
}

impl LlmChatCompletion for Ollama {
    async fn chat_complete(&self, messages: Vec<Message>) -> Result<Message, Error> {
        let messages: Result<Vec<ChatMessage>, _> = messages
            .into_iter()
            .map(|message| message.try_into())
            .collect();

        let messages = messages?;

        let request = ChatMessageRequest::new(self.model.clone(), messages);
        let response = self.ollama.send_chat_messages(request).await?;

        if let Some(message) = response.message {
            Ok(message.into())
        } else {
            bail!("No message");
        }
    }
}

impl LlmCompletion for Ollama {
    async fn complete(&self, prompt: impl ToString) -> Result<String, Error> {
        let request = GenerationRequest::new(self.model.clone(), prompt.to_string());
        let response = self.ollama.generate(request).await?;

        Ok(response.response)
    }
}
