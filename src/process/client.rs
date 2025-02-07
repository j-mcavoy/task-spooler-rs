use super::msg::Msg;
use anyhow::Result;
use bincode::serialize;
use thiserror::Error;
use uds::UnixSeqpacketConn;

#[derive(Default)]
struct Client {
  conn: Option<UnixSeqpacketConn>,
}
impl Client {
  pub fn send_msg(&self, msg: Msg) -> Result<usize, anyhow::Error> {
    if let Some(conn) = &self.conn {
      let bytes: Vec<u8> = serialize(&msg)?;
      Ok(conn.send(bytes.as_slice())?)
    } else {
      Err(ClientError::NotConnected.into())
    }
  }

  pub fn connect(&mut self) -> anyhow::Result<()> {
    self.conn = Some(uds::UnixSeqpacketConn::connect(super::SOCK)?);
    Ok(())
  }
}

#[derive(Error, Debug)]
enum ClientError {
  #[error("not connected to socket")]
  NotConnected,
}
#[cfg(test)]
use serial_test::serial;
mod tests {
  use super::{super::server::Server, *};
  #[test]
  #[serial]
  fn connect() -> anyhow::Result<()> {
    let mut s = Server::default();
    let mut c = Client::default();
    s.start()?;
    c.connect()?;
    s.stop()?;
    Ok(())
  }
  #[test]
  #[serial]
  fn send_msg() -> anyhow::Result<()> {
    let mut s = Server::default();
    let mut c = Client::default();

    s.start()?;
    c.connect()?;
    c.send_msg(Msg::KillAll(None))?;
    s.stop()?;
    Ok(())
  }
}
