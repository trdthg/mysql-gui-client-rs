use std::ops::{Deref, DerefMut};

use crate::apps::Connection;
use crate::service::api::mysql::ConnectionConfig;
use crate::util::duplex_channel;
use crate::util::duplex_channel::{DuplexConsumer, DuplexProducer};

pub struct DatabaseServer {
    pub inner: DuplexProducer<(ConnectionConfig, bool), Connection>,
}

pub struct DatabaseClient {
    pub inner: DuplexConsumer<(ConnectionConfig, bool), Connection>,
}

pub fn make_db_service() -> (DatabaseClient, DatabaseServer) {
    let (sql_sender, sql_executor) =
        duplex_channel::channel::<(ConnectionConfig, bool), Connection>();
    let client = DatabaseClient { inner: sql_sender };
    let handler = DatabaseServer {
        inner: sql_executor,
    };
    (client, handler)
}
