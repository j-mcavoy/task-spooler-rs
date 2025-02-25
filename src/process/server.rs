use super::msg::Msg;
use bincode::deserialize;
use log::*;
use thiserror::Error;
use tokio_seqpacket::{UnixSeqpacket, UnixSeqpacketListener};
pub mod env;
mod handle;
use env::*;
mod queue;
use queue::Queue;

pub struct Exe;

#[derive(Default)]
pub struct Server {
  listner: Option<UnixSeqpacket>,
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

  pub async fn connect(&mut self) -> anyhow::Result<()> {
    debug!("connecting server");
    let _ = UnixSeqpacketListener::bind(super::SOCK);
    if let Some(listner) = Some(UnixSeqpacket::connect(super::SOCK).await?) {
      self.listner = Some(listner);
      debug!("connected");
      Ok(())
    } else {
      Err(ServerError::NotStarted.into())
    }
  }

  pub async fn restart(&mut self) -> anyhow::Result<()> {
    self.stop()?;
    self.connect().await?;
    Ok(())
  }

  pub fn stop(&mut self) -> anyhow::Result<()> {
    self
      .listner
      .as_mut()
      .unwrap()
      .shutdown(std::net::Shutdown::Both)
      .unwrap();
    self.listner = None;
    debug!("Stopping server");
    Ok(std::fs::remove_file(super::SOCK)?)
  }

  pub async fn run(&mut self) -> anyhow::Result<()> {
    let listner = self.listner.as_ref().expect("not connected");

    debug!("running..");
    loop {
      let buffer = &mut [0; 12];
      let _ = listner.recv(buffer).await;
      debug!("server: {buffer:?}");
      let msg: Msg = deserialize(buffer)?;
      debug!("server: {msg:?}");
      if msg == Msg::KillServer {
        break;
      }
    }
    self.stop()
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
