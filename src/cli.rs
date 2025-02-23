mod action;
use crate::process::msg::Msg;
use action::*;
use clap::*;

#[derive(Parser)]
#[command(version, about)]
#[clap(trailing_var_arg = true)]
pub struct Cli {
  /// Actions
  ///
  /// Actions:  [-KClhVTRqM]  [-t  [id]]  [-c  [id]]  [-p [id]] [-o [id]] [-s [id]] [-r [id]] [-w [id]] [-k [id]] [-u [id]] [-i [id]] [-U <id-id>] [-S [num]] [-a/--get_label [id]] [-F/--full_cmd [id]] [--getenv [var]] [--setenv [var=val]] [--unsetenv [var]] [--get_logdir]
  #[command(flatten)]
  action: Action,

  #[arg(group = "options")]
  options: Option<Vec<Opt>>,

  /// executable command to queue
  #[arg(trailing_var_arg = true, allow_hyphen_values = true, group = "input")]
  command: Option<Vec<String>>,
}

enum ProcessType {
  Client,
  Server,
}

pub fn cli() -> Cli {
  let cli = Cli::parse();
  return cli;
}
