use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    System,
    User,
    Assistant,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub role: MessageRole,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageRequestPayload {
    pub model: String,
    pub messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageResponseUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageResponseChoiceMessage {
    pub role: MessageRole,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageResponseChoice {
    pub message: MessageResponseChoiceMessage,
    pub finish_reason: Option<String>,
    pub index: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageResponsePayload {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub usage: MessageResponseUsage,
    pub choices: Vec<MessageResponseChoice>,
}
