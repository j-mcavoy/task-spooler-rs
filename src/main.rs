#![feature(never_type)]
#![feature(try_trait_v2)]
use log::debug;
use process::{
  client::Client,
  msg::Msg,
  server::{env::Env, Server},
};

mod cli;
//mod ffi;
mod process;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  env_logger::init();
  let env = Env::default();
  let cli = cli::cli();
  let mut server = Server::new(env);
  let mut client = Client::default();

  if server.connect().is_err() {
    server.restart()?;
  };

  server.run()?;

  client.connect()?;

  client.send_msg(Msg::KillAll(None))?;
  client.send_msg(Msg::SwapJobs((45, 54)))?;
  client.send_msg(Msg::KillServer)?;
  server.stop()?;
  Ok(())
}
