use anyhow::Result;
use std::io::Write;
use std::net::{SocketAddr, TcpStream};

use self::message::Message;
pub mod message;
pub struct Client {
    server_addr: SocketAddr,
    conn: TcpStream,
}

impl Client {
    pub fn new(addr: impl Into<SocketAddr>) -> Result<Self> {
        let addr = addr.into();
        let conn = TcpStream::connect(addr)?;
        Ok(Self {
            server_addr: addr,
            conn,
        })
    }

    fn do_message(&mut self, msg: String) -> Result<()> {
        self.conn.write(msg.as_bytes())?;
        self.conn.write_all("\n".as_bytes())?;
        Ok(())
    }

    pub fn connect(&mut self) -> Result<()> {
        let conn = TcpStream::connect(self.server_addr)?;
        self.conn = conn;
        Ok(())
    }

    pub fn send_msg(&mut self, msg: Message) -> Result<()> {
        match msg {
            Message::Normal { msg, user } => self.do_message(msg),
            Message::Cmd(cmd) => match cmd {
                message::Cmd::Who => self.get_active_users(),
                message::Cmd::Secret(_, _) => self.send_to(),
            },
        }
    }

    fn get_active_users(&mut self) -> Result<()> {
        //
        Ok(())
    }
    fn send_to(&mut self) -> Result<()> {
        //
        Ok(())
    }

    pub(crate) fn read_new_msgs(&self) -> Vec<Message> {
        todo!()
    }
}
