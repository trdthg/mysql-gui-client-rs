use std::pin::Pin;

use tokio::sync::{broadcast, mpsc};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tonic::{
    codegen::futures_core::Stream, transport::Server, Extensions, Request, Response, Status,
};
use tracing::{info, log::warn};

use crate::{
    chat_server::ChatServer, pb::chat_server::Chat, ChatMessage, SendMessageResponse, Token,
};

struct ChatService {
    tx: broadcast::Sender<ChatMessage>,
}

impl Default for ChatService {
    fn default() -> Self {
        let (tx, _rx) = broadcast::channel(MAX_MESSAGES);
        Self { tx }
    }
}

#[tonic::async_trait]
impl Chat for ChatService {
    ///Server streaming response type for the GetMessages method.
    async fn login(
        &self,
        request: tonic::Request<super::LoginRequest>,
    ) -> Result<tonic::Response<super::Token>, tonic::Status> {
        let info = request.into_inner();
        info!("login: {:?}", info);
        Ok(Response::new(info.into_token()))
    }
    /// send message to a room
    async fn send_message(
        &self,
        request: tonic::Request<super::NewChatMessage>,
    ) -> Result<tonic::Response<super::SendMessageResponse>, tonic::Status> {
        // TODO: how to get sender
        let sender = get_username(&request.extensions())?;
        let info = request.into_inner();
        let msg = info.into_chat_message(sender);
        info!("send message: {:?}", info);
        // TODO: store the message
        // TODO: how to publish to every one who interested in it
        self.tx.send(msg).unwrap();
        Ok(Response::new(SendMessageResponse {}))
    }
    ///Server streaming response type for the GetMessages method.
    type GetMessagesStream =
        Pin<Box<dyn Stream<Item = Result<super::ChatMessage, tonic::Status>> + Send>>;
    /// subscribe and get all message
    async fn get_messages(
        &self,
        request: tonic::Request<super::GetMessageRequest>,
    ) -> Result<tonic::Response<Self::GetMessagesStream>, tonic::Status> {
        let info = request.into_inner();
        info!("send message: {:?}", info);
        let mut tx = self.tx.subscribe();
        let (sender, receiver) = mpsc::unbounded_channel();

        tokio::spawn(async move {
            while let Ok(msg) = tx.recv().await {
                if let Err(e) = sender.send(Ok(msg)) {
                    warn!("Failed to send: {}", e);
                    break;
                }
            }
        });
        let stream = UnboundedReceiverStream::new(receiver);
        Ok(Response::new(Box::pin(stream)))
    }
}

const MAX_MESSAGES: usize = 100;

pub async fn start() {
    let svc = ChatServer::with_interceptor(ChatService::default(), chat_auth);
    let addr = "0.0.0.0:8080".parse().unwrap();
    info!("listening on http://{}", addr);
    Server::builder()
        .add_service(svc)
        .serve(addr)
        .await
        .unwrap();
}

fn chat_auth(mut req: Request<()>) -> Result<Request<()>, tonic::Status> {
    let token = match req.metadata().get("authorization") {
        Some(v) => {
            let data = v
                .to_str()
                .map_err(|_| Status::new(tonic::Code::Unauthenticated, "Invalid token format"))?;
            Token::new(data.strip_prefix("Bearer ").unwrap())
        }
        None => {
            Token::default()
            // return Err(Status::new(
            //     tonic::Code::Unauthenticated,
            //     "Missing authorization header",
            // ))
        }
    };
    req.extensions_mut().insert(token);
    Ok(req)
}

fn get_username(ext: &Extensions) -> Result<String, Status> {
    let token = ext
        .get::<Token>()
        .ok_or(Status::unauthenticated("No Token"))?;
    if token.is_valid() {
        Ok(token.into_username())
    } else {
        Err(Status::unauthenticated("Invalid Token"))
    }
}
