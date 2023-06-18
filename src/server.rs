#[cfg(feature = "article")]
pub mod article;
#[cfg(feature = "database")]
pub mod database;
#[cfg(feature = "talk")]
pub mod talk;

#[cfg(feature = "article")]
use article::ArticleClient;
use async_trait::async_trait;
#[cfg(feature = "database")]
use database::DatabaseClient;
use tokio::sync::mpsc::{
    error::{SendError, TryRecvError},
    unbounded_channel, UnboundedReceiver, UnboundedSender,
};

#[async_trait]
pub trait Server: Send + Sync + 'static {
    async fn block_on(&mut self);
}

pub trait Client: Send + Sync + 'static {
    type S;
    type D;

    fn send(&self, singal: Self::S) -> Result<(), SendError<Self::S>>;
    fn try_recv(&mut self) -> Result<Self::D, TryRecvError>;
}

pub struct Channels<S, D> {
    c_s: UnboundedSender<S>,
    c_r: UnboundedReceiver<D>,
    s_s: UnboundedSender<D>,
    s_r: UnboundedReceiver<S>,
}

pub fn make_chan<S, D>() -> Channels<S, D> {
    let (client_sender, server_receiver) = unbounded_channel::<S>();
    let (server_sender, client_receiver) = unbounded_channel::<D>();
    Channels {
        c_s: client_sender,
        c_r: client_receiver,
        s_s: server_sender,
        s_r: server_receiver,
    }
}

pub struct Repo {
    #[cfg(feature = "article")]
    pub article_client: ArticleClient,
    #[cfg(feature = "database")]
    pub database_client: DatabaseClient,
}

pub struct Backend {
    pub servers: Vec<Box<dyn Server>>,
}

impl Backend {
    pub fn run(self) {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_io()
            .enable_time()
            .build()
            .unwrap();
        runtime.block_on(async move {
            let mut handlers = vec![];
            for mut producer in self.servers {
                let t = tokio::task::spawn(async move {
                    producer.block_on().await;
                });
                handlers.push(t);
            }
            for handler in handlers {
                if let Err(e) = handler.await {
                    tracing::error!("tokio 任务执行失败：{}", e);
                }
            }
        });
    }
}
