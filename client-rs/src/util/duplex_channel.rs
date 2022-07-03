use std::{future::Future, pin::Pin};

use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

pub struct DuplexConsumer<S, D> {
    signal_chan: UnboundedSender<S>,
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
    data_chan: UnboundedSender<D>,
}

unsafe impl<D, S> Send for DuplexConsumer<S, D> {}
unsafe impl<D, S> Send for DuplexProducer<S, D> {}

impl<S, D> DuplexProducer<S, D> {
    pub fn take(self) -> Self {
        self
    }
    pub async fn wait_produce<F, Fut>(&mut self, f: F)
    where
        F: Fn() -> Pin<Box<Fut>>,
        Fut: Future<Output = D>,
    {
        loop {
            tracing::debug!("等待信号");
            match self.signal_chan.recv().await {
                None => {
                    // tracing::error!("信号接收失败：{}", e);
                    tracing::error!("信号接收失败：");
                }
                Some(_) => {
                    let res = f().await;
                    if let Err(e) = self.data_chan.send(res) {
                        // tracing::error!("发送请求结果失败：{:?}", e);
                        tracing::error!("发送请求结果失败");
                    }
                    tracing::debug!("发送请求结果成功");
                }
            }
        }
    }

    pub async fn wait_handle_produce<F, Fut>(&mut self, f: F)
    where
        F: Fn(S) -> Pin<Box<Fut>>,
        Fut: Future<Output = D>,
    {
        tracing::debug!("等待信号");
        loop {
            match self.signal_chan.recv().await {
                None => {
                    // tracing::error!("信号接收失败：{}", e);
                    tracing::error!("信号接收失败");
                }
                Some(signal) => {
                    let res = f(signal).await;
                    if let Err(e) = self.data_chan.send(res) {
                        // tracing::error!("发送请求结果失败：{:?}", e);
                        tracing::error!("发送请求结果失败");
                        // break;
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
