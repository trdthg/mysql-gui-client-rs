use std::sync::Arc;

use arc_swap::ArcSwap;
use dashmap::DashMap;
// use std::ops::Deref;
use tonic::{
    codegen::InterceptedService, metadata::AsciiMetadataValue, service::Interceptor,
    transport::Channel,
};
use tracing::info;

use crate::{
    chat_client::ChatClient, ChatMessage, GetMessageRequest, LoginRequest, NewChatMessage, Token,
};
use anyhow::Result;

lazy_static::lazy_static! {
    static ref TOKEN: ArcSwap<Token> = ArcSwap::from(
        Arc::new(Token {..Default::default()})
    );
}

// 键：room
// 值：该 room 的所有 message
#[derive(Default, Clone)]
struct Rooms(Arc<DashMap<String, Vec<ChatMessage>>>);

impl Rooms {
    fn insert_message(&self, msg: ChatMessage) {
        self.0.entry(msg.room.clone()).or_insert(vec![]).push(msg);
    }
}

// impl Deref for Rooms {
//     type Target = DashMap<String, Vec<ChatMessage>>;

//     fn deref(&self) -> &Self::Target {
//         todo!()
//     }
// }

pub struct Client {
    username: String,
    conn: ChatClient<InterceptedService<Channel, AuthInterceptor>>,
    rooms: Rooms,
}

impl Client {
    pub async fn new(username: impl Into<String>) -> Self {
        let channel = Channel::from_static("http://127.0.0.1:8080")
            .connect()
            .await
            .unwrap();
        let client = ChatClient::with_interceptor(channel, AuthInterceptor::default());
        Self {
            username: username.into(),
            conn: client,
            rooms: Default::default(),
        }
    }

    pub async fn login(&mut self) -> Result<()> {
        let login = LoginRequest::new(self.username.clone(), "passwor");
        let token = self.conn.login(login).await?.into_inner();
        TOKEN.store(Arc::new(token));
        Ok(())
    }

    pub async fn send_message(
        &mut self,
        room: impl Into<String>,
        content: impl Into<String>,
    ) -> Result<()> {
        let msg = NewChatMessage::new(room, content);
        self.conn.send_message(msg).await?;
        Ok(())
    }

    pub async fn get_messages(&mut self) -> Result<()> {
        let req = GetMessageRequest::new();
        let mut stream = self.conn.get_messages(req).await?.into_inner();
        let rooms = self.rooms.clone();
        tokio::spawn(async move {
            while let Some(msg) = stream.message().await? {
                // if msg.sender == self.username {
                //     continue;
                // }
                info!(
                    "got message: {:?} said:{} at {}",
                    msg.sender, msg.content, msg.timestamp
                );
                rooms.insert_message(msg);
            }
            // TODO: 不知道为什么
            Ok::<_, tonic::Status>(())
        });
        Ok(())
    }
}

#[derive(Default)]
struct AuthInterceptor;

impl Interceptor for AuthInterceptor {
    fn call(
        &mut self,
        mut request: tonic::Request<()>,
    ) -> Result<tonic::Request<()>, tonic::Status> {
        let token = TOKEN.load();
        info!(
            "send to: {:?} with token: {}",
            request.metadata(),
            token.data
        );
        if !token.is_valid() {
            return Ok(request);
        }
        let value = AsciiMetadataValue::try_from(format!("Bearer {}", token.data)).unwrap();
        request.metadata_mut().insert("authorization", value);
        Ok(request)
    }
}
