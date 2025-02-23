use super::msg::Msg;
use bincode::deserialize;
use log::*;
use thiserror::Error;
use uds::UnixSeqpacketListener;
pub mod env;
mod handle;
use env::*;
mod queue;
use queue::Queue;

pub struct Exe;

#[derive(Default)]
pub struct Server {
  listner: Option<UnixSeqpacketListener>,
  env: Env,
  queue: Queue,
}
impl Server {
  pub fn new(env: Env) -> Self {
    Self {
      listner: None,
      env,
      queue: Default::default(),
    }
  }

  pub fn connect(&mut self) -> anyhow::Result<()> {
    if let Some(listner) = Some(UnixSeqpacketListener::bind(super::SOCK)?) {
      self.listner = Some(listner);
      Ok(())
    } else {
      Err(ServerError::NotStarted.into())
    }
  }

  pub fn restart(&mut self) -> anyhow::Result<()> {
    self.stop()?;
    self.connect()?;
    Ok(())
  }
  pub fn stop(&mut self) -> anyhow::Result<()> {
    self.listner = None;
    debug!("Stopping server");
    Ok(std::fs::remove_file(super::SOCK)?)
  }

  pub fn run(&mut self) -> anyhow::Result<()> {
    let (conn, _addr) = self
      .listner
      .as_ref()
      .expect("not connected")
      .accept_unix_addr()?;

    loop {
      let buffer = &mut [0; 12];
      let _ = conn.recv(buffer)?;
      debug!("server: {buffer:?}");
      let msg: Msg = deserialize(buffer)?;
      debug!("server: {msg:?}");
      if msg == Msg::KillServer {
        break;
      }
    }
    Ok(self.stop()?)
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
    s.connect()?;
    s.stop()?;
    Ok(())
  }
}
