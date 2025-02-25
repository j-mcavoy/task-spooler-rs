pub mod action;
mod opt;
use crate::process::msg::{ListFormat, Msg};
use action::*;
use clap::*;
use opt::*;
use serde::{Deserialize, Serialize};

pub type Var = String;

#[derive(Parser, Debug)]
#[command(name = "tsp", author = "John McAvoy", version, about)]
#[clap(trailing_var_arg = true)]
pub struct Cli {
  /// Actions
  ///
  /// Actions:  [-KClhVTRqM]  [-t  [id]]  [-c  [id]]  [-p [id]] [-o [id]] [-s [id]] [-r [id]] [-w [id]] [-k [id]] [-u [id]] [-i [id]] [-U <id-id>] [-S [num]] [-a/--get_label [id]] [-F/--full_cmd [id]] [--getenv [var]] [--setenv [var=val]] [--unsetenv [var]] [--get_logdir]
  #[command(flatten)]
  pub action: Option<Action>,

  #[command(flatten)]
  pub options: Option<Opt>,

  /// executable command to queue
  #[arg(
    required = false,
    trailing_var_arg = true,
    allow_hyphen_values = true,
    group = "action"
  )]
  pub command: Option<Vec<String>>,
}

enum ProcessType {
  Client,
  Server,
}

#[rustfmt::skip]
impl crate::cli::Cli {
  pub fn to_msg(self) -> Msg {
    let Cli {
      action,
      options,
      command,
    } = self;

    match (action, options, command) {
      (Some(a), _, _) => match a {
          Action { GetEnv: Some(e), .. } => Msg::GetEnv(e),
          Action { SetEnv: Some(e), .. } => Msg::SetEnv(e.clone(), e),
          Action { UnsetEnv: Some(e), .. } => Msg::UnsetEnv(e),
          Action { KillAll: true, .. } => Msg::KillAll,
          Action { KillServer: true, .. } => Msg::KillServer,
          Action { ClearFinished: true, .. } => Msg::ClearFinished,
          Action { List: true, .. } => Msg::List(ListFormat::Default),
          Action { Serialize: Some(fmt), .. } => Msg::List(fmt),
          Action { LastId: true, .. } => Msg::LastId,
          Action { CountRunning: true, .. } => Msg::CountRunning,
          Action { GetLabel: Some(e), .. } => Msg::GetLabel(e),
          Action { FullCmd: Some(job_id), .. } => unimplemented!(),
          Action { GetLogDir: true, .. } => Msg::GetLogdir,
          Action { SetLogDir: Some(path), .. } => Msg::SetLogdir(path),
          Action { Tail: Some(job_id), .. } => Msg::Tail(job_id),
        _ => unimplemented!()
      },
      (None, None, None) => Msg::List(ListFormat::Default),
      _ => unimplemented!()
    }
  }
}
