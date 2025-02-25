#![feature(never_type)]
#![feature(try_trait_v2)]
mod cli;
mod process;
use bincode::serialize;
use clap::{Args, Parser, Subcommand, ValueEnum};
use cli::{action::Action, Cli};
use log::debug;
use process::{
  client::Client,
  msg::Msg,
  server::{env::Env, Server},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  env_logger::init();
  let env = Env::default();
  let mut client = Client::default();

  let cli = Cli::parse();

  debug!("{cli:?}");

  if client.connect().is_err() {
    let mut server = Server::new(env);
    if server.connect().await.is_err() {
      debug!("Starting server");
      server.connect().await?;
      tokio::spawn(async move {
        server.run().await.unwrap();
      });
    };
  }

  client.connect()?;

  let msg = cli.to_msg();
  debug!("{msg:?}");
  debug!("msg: {:?}", serialize(&msg));
  client.send_msg(msg)?;
  client.send_msg(Msg::KillServer)?;

  Ok(())
}
