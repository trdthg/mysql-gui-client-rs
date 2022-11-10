mod abi;
pub use abi::*;

impl LoginRequest {
    pub fn new(username: impl Into<String>, password: impl Into<String>) -> Self {
        Self {
            username: username.into(),
            password: password.into(),
        }
    }

    pub fn into_token(&self) -> Token {
        Token::new(self.username.clone())
    }
}

impl Token {
    // TODO! use jwt
    pub fn new(data: impl Into<String>) -> Self {
        Self { data: data.into() }
    }

    // TODO! use jwt for decode username
    pub fn into_username(&self) -> String {
        self.data.clone()
    }

    pub fn is_valid(&self) -> bool {
        !self.data.is_empty()
    }
}

impl NewChatMessage {
    pub fn new(room: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            room: room.into(),
            content: content.into(),
        }
    }

    pub fn into_chat_message(&self, sender: impl Into<String>) -> ChatMessage {
        let timestamp = chrono::Utc::now().timestamp();
        ChatMessage {
            sender: sender.into(),
            room: self.room.clone(),
            content: self.content.clone(),
            timestamp,
        }
    }
}

impl ChatMessage {
    pub fn new(
        sender: impl Into<String>,
        room: impl Into<String>,
        contnet: impl Into<String>,
    ) -> Self {
        let timestamp = chrono::Utc::now().timestamp();
        Self {
            sender: sender.into(),
            room: room.into(),
            content: contnet.into(),
            timestamp,
        }
    }
}

impl GetMessageRequest {
    pub fn new() -> Self {
        Self {}
    }
}
