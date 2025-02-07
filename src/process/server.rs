use super::msg::Msg;
use bincode::deserialize;
use log::*;
use thiserror::Error;
use uds::UnixSeqpacketListener;

#[derive(Default)]
pub struct Server {
  listner: Option<UnixSeqpacketListener>,
}
impl Server {
  pub fn start(&mut self) -> anyhow::Result<()> {
    self.listner = Some(UnixSeqpacketListener::bind(super::SOCK)?);
    Ok(())
  }

  pub fn stop(&mut self) -> anyhow::Result<()> {
    self.listner = None;
    Ok(std::fs::remove_file(super::SOCK)?)
  }

  pub fn run(&self) -> anyhow::Result<()> {
    if let Some(listner) = &self.listner {
      let (conn, _addr) = listner.accept_unix_addr()?;

      loop {
        let mut buffer = &mut [0; 50];
        let _ = conn.recv(buffer)?;
        let msg: Msg = deserialize(buffer)?;
        debug!("{msg:?}");
      }
    } else {
      return Err(ServerError::NotStarted.into());
    }
  }
}

#[derive(Error, Debug)]
enum ServerError {
  #[error("not connected to socket")]
  NotStarted,
}

#[cfg(test)]
mod tests {
  use super::*;
  use serial_test::serial;
  #[test]
  #[serial]
  fn start_stop() -> anyhow::Result<()> {
    let mut s = Server::default();
    s.start()?;
    s.stop()?;
    Ok(())
  }
}
