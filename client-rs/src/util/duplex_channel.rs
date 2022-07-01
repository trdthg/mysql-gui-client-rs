use std::{
    future::Future,
    pin::Pin,
    sync::mpsc::{self, Receiver, Sender},
};

pub struct DuplexConsumer<T> {
    data_chan: Receiver<T>,
    signal_chan: Sender<()>,
}

impl<T> DuplexConsumer<T> {
    pub fn send(&mut self) -> Result<(), mpsc::SendError<()>> {
        self.signal_chan.send(())
    }
    pub fn try_recv(&mut self) -> Result<T, mpsc::TryRecvError> {
        self.data_chan.try_recv()
    }
}

pub struct DuplexProducer<T> {
    data_chan: Sender<T>,
    signal_chan: Receiver<()>,
}

unsafe impl<T: Send> Send for DuplexConsumer<T> {}
unsafe impl<T: Send> Send for DuplexProducer<T> {}

impl<T> DuplexProducer<T> {
    pub async fn wait_for_produce<F, Fut>(&mut self, f: F)
    where
        F: Fn() -> Pin<Box<Fut>>,
        Fut: Future<Output = T>,
    {
        loop {
            if let Err(_) = self.signal_chan.recv() {
                break;
            }
            let res = f().await;
            if let Err(e) = self.data_chan.send(res) {
                tracing::error!("Channel 发送数据失败：{:?}", e);
                break;
            }
            tracing::debug!("Channel 发送数据成功");
        }
    }
}

pub fn channel<T>() -> (DuplexConsumer<T>, DuplexProducer<T>) {
    let (sign_sender, sign_receiver) = mpsc::channel::<()>();
    let (data_sender, data_receiver) = mpsc::channel::<T>();
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
