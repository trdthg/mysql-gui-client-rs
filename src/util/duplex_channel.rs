use std::{fmt::Debug, future::Future};

use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

pub struct DuplexConsumer<S, D> {
    pub signal_chan: UnboundedSender<S>,
    data_chan: UnboundedReceiver<D>,
}

impl<S, D> DuplexConsumer<S, D> {
    pub fn send(&mut self, signal: S) -> Result<(), tokio::sync::mpsc::error::SendError<S>> {
        self.signal_chan.send(signal)
    }

    pub fn try_recv(&mut self) -> Result<D, tokio::sync::mpsc::error::TryRecvError> {
        self.data_chan.try_recv()
    }
}

pub struct DuplexProducer<S, D> {
    signal_chan: UnboundedReceiver<S>,
    pub data_chan: UnboundedSender<D>,
}

unsafe impl<D, S> Send for DuplexConsumer<S, D> {}
unsafe impl<D, S> Sync for DuplexConsumer<S, D> {}
unsafe impl<D, S> Send for DuplexProducer<S, D> {}
unsafe impl<D, S> Sync for DuplexProducer<S, D> {}

impl<S, D> DuplexProducer<S, D>
where
    D: Debug,
    S: Debug,
{
    pub fn send(&mut self, data: D) -> Result<(), tokio::sync::mpsc::error::SendError<D>> {
        self.data_chan.send(data)
    }

    pub async fn try_recv(&mut self) -> Option<S> {
        self.signal_chan.recv().await
    }

    pub async fn wait_produce<F, Fut>(&mut self, f: F)
    where
        // F: Fn() -> Pin<Box<Fut>>,
        F: Fn() -> Fut,
        Fut: Future<Output = D> + Send + 'static,
    {
        loop {
            match self.signal_chan.recv().await {
                None => {
                    tracing::error!("发送方已关闭");
                }
                Some(_) => {
                    let res = f().await;
                    if let Err(e) = self.data_chan.send(res) {
                        tracing::error!("发送连接结果失败，GUI 可能停止工作：{}", e);
                        continue;
                    }
                    tracing::debug!("发送请求结果成功");
                }
            }
        }
    }

    pub async fn wait_handle_produce<F, Fut>(&mut self, f: F)
    where
        F: Fn(S) -> Fut,
        Fut: Future<Output = D> + Send + 'static,
    {
        loop {
            match self.signal_chan.recv().await {
                None => {
                    tracing::error!("发送方已关闭");
                }
                Some(signal) => {
                    let res = f(signal).await;
                    if let Err(e) = self.data_chan.send(res) {
                        tracing::error!("发送执行结果失败，GUI 可能停止工作： {}", e);
                        continue;
                    }
                    tracing::debug!("发送请求结果成功");
                }
            }
        }
    }
}

pub fn channel<S, D>() -> (DuplexConsumer<S, D>, DuplexProducer<S, D>) {
    let (sign_sender, sign_receiver) = unbounded_channel::<S>();
    let (data_sender, data_receiver) = unbounded_channel::<D>();
    (
        DuplexConsumer {
            data_chan: data_receiver,
            signal_chan: sign_sender,
        },
        DuplexProducer {
            data_chan: data_sender,
            signal_chan: sign_receiver,
        },
    )
}
