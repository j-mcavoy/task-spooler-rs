use super::{Cli, Var};
use crate::process::{job::JobId, msg::*};
use clap::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[allow(non_snake_case)]
#[derive(Args, Debug)]
#[group(required = false, multiple = false)]
pub struct Action {
  /// Get the specified environment variable value from the tsp server.
  #[arg(required = false, long = "getenv", value_name = "ENV")]
  pub GetEnv: Option<Var>,

  /// Set the specified environment variable value from the tsp server.
  #[arg(required = false, long = "setenv", value_name = "ENV")]
  pub SetEnv: Option<Var>,

  /// Remove the specified environment variable from the tsp server. Set the specified environment variable to the tsp server.
  #[arg(required = false, long = "unsetenv", value_name = "ENV")]
  pub UnsetEnv: Option<Var>,

  /// Kill the tsp server for the calling client. This will remove the unix socket and all the tsp processes related to the queue. This will not kill the command being run at that time.
  ///
  /// It is not reliable to think that tsp -K will finish when the server is really killed. By now it is a race condition.
  #[arg(required = false, short = 'K')]
  pub KillServer: bool,

  /// Send SIGTERM to all running job groups.
  #[arg(required = false, short = 'T')]
  pub KillAll: bool,

  /// Clear the results of finished jobs from the queue.
  #[arg(required = false, short = 'C')]
  pub ClearFinished: bool,

  /// Show the list of jobs - to be run, running and finished - for the current queue.  This is the default behaviour if tsp is called without options.
  #[arg(required = false, short = 'l')]
  pub List: bool,

  /// Serialize the job list to the specified format. Choices: {default, json, tab}.
  #[arg(required = false, value_enum, long = "serialize", short = 'M')]
  pub Serialize: Option<ListFormat>,

  /// Show the job ID of the last added.
  #[arg(required = false, long = "last_queue_id", short = 'q')]
  pub LastId: bool,

  /// Return the number of running jobs
  #[arg(required = false, long = "count_running", short = 'R')]
  pub CountRunning: bool,

  /// Show the job label. Of the last added, if not specified.
  #[arg(required = false, long = "get_label", short = 'a', value_name = "id")]
  pub GetLabel: Option<Option<JobId>>,

  /// Show the full command. Of the last added, if not specified.
  #[arg(required = false, long = "full_cmd", short = 'F', value_name = "id")]
  pub FullCmd: Option<Option<JobId>>,

  /// Show the path containing log files.
  #[arg(required = false, long = "get_logdir")]
  pub GetLogDir: bool,

  /// Set the path containing log files to the specified path.
  #[arg(
    required = false,
    required = false,
    long = "set_logdir",
    value_name = "path"
  )]
  pub SetLogDir: Option<PathBuf>,

  /// Show the last ten lines of the output file of the named job, or the last running/run if not specified. If the job is still running, it will keep on showing the additional output until the job finishes. On exit, it returns the errorlevel of the job, as in -c.
  #[arg(required = false, short = 't', value_name = "id")]
  pub Tail: Option<Option<JobId>>,

  /// Run the system's cat to the output file of the named job, or the last running/run if not specified. It will block until all the output can be sent to standard output, and will exit with the job errorlevel as in -c.
  #[arg(required = false, short = 'c', value_name = "id")]
  pub Cat: Option<JobId>,

  /// Show the pid of the named job, or the last running/run if not specified.
  #[arg(required = false, short = 'p', value_name = "id")]
  pub ShowPid: Option<JobId>,

  /// Show the output file name of the named job, or the last running/run if not specified.
  #[arg(required = false, short = 'o', value_name = "id")]
  pub OutputFile: Option<JobId>,

  /// Show the job state of the named job, or the last in the queue.
  #[arg(required = false, short = 's', value_name = "id")]
  pub ShowState: Option<JobId>,

  /// Remove the named job, or the last in the queue.
  #[arg(required = false, short = 'r', value_name = "id")]
  pub Remove: Option<JobId>,

  /// Wait for the named job, or for the last in the queue.
  #[arg(required = false, short = 'w', value_name = "id")]
  pub WaitFor: Option<JobId>,

  /// Kill the process group of the named job (SIGTERM), or the last running/run job if not specified.  Equivalent to kill -- -‘tsp -p‘
  #[arg(required = false, short = 'k', value_name = "id")]
  pub Kill: Option<JobId>,

  /// Make the named job (or the last in the queue) urgent - this means that it goes forward in the queue so it can run as soon as possible.
  #[arg(required = false, short = 'u', value_name = "id")]
  pub Urgent: Option<JobId>,

  /// Show information about the named job (or the last run). It will show the command line, some times related to the task, and also any information resulting from TS_ENV (Look at ENVIRONMENT).
  #[arg(required = false, short = 'i', value_name = "id")]
  pub Info: Option<JobId>,
  /// -U <id-id>
  /// Interchange the queue positions of the named jobs (separated by a hyphen and no spaces).
  #[arg(required = false, short = 'U', value_name = "id-id")]
  pub Interchange: Option<InterchangeS>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct InterchangeS {
  ids: String,
}

impl From<String> for InterchangeS {
  fn from(value: String) -> InterchangeS {
    InterchangeS { ids: value }
  }
}

impl TryInto<(JobId, JobId)> for InterchangeS {
  type Error = String;
  fn try_into(self) -> std::result::Result<(JobId, JobId), Self::Error> {
    if let Some((lhs, rhs)) = self.ids.split_once("-") {
      if let (Ok(lhs), Ok(rhs)) = (
        JobId::from_str_radix(lhs, 10),
        JobId::from_str_radix(rhs, 10),
      ) {
        return Ok((lhs, rhs));
      }
    }
    Err("Not 2 id's".into())
  }
}

/*
#[allow(non_snake_case)]
#[derive(Subcommand, Debug, Serialize, Deserialize)]
#[group(required = false)]
pub enum Action {
  /// Get the specified environment variable value from the tsp server.
  #[command(name = "--getenv")]
  GetEnv(GetEnv),

  /// Get the specified environment variable value from the tsp server.
  #[command(name = "--setenv")]
  SetEnv(SetEnv),

  /// Remove the specified environment variable from the tsp server. Set the specified environment variable to the tsp server.
  #[command(name = "--unsetenv")]
  UnsetEnv,

  /// Kill the tsp server for the calling client. This will remove the unix socket and all the tsp processes related to the queue. This will not kill the command being run at that time.
  ///
  /// It is not reliable to think that tsp -K will finish when the server is really killed. By now it is a race condition.
  #[command(smhort = 'K')]
  KillAll,

  /// Send SIGTERM to all running job groups.
  #[command(short = 'T')]
  Sigterm,

  /// Clear the results of finished jobs from the queue.
  #[command(short = 'C')]
  Clear,

  /// Show the list of jobs - to be run, running and finished - for the current queue.  This is the default behaviour if tsp is called without options.
  #[command(short = 'l')]
  List,

  /// Serialize the job list to the specified format. Choices: {default, json, tab}.
  #[command(long = "serialize", short = 'M')]
  Serialize(Option<ListFormat>),

  /// Show the job ID of the last added.
  #[command(long = "last_queue_id", short = 'q')]
  LastQueue,

  /// Return the number of running jobs
  #[command(long = "count_running", short = 'R')]
  CountRunning,

  /// Show the job label. Of the last added, if not specified.
  #[command(long = "get_label", short = 'a', value_name = "id")]
  GetLabel(Option<JobId>),

  /// Show the full command. Of the last added, if not specified.
  #[command(long = "full_cmd", short = 'F', value_name = "id")]
  FullCmd(Option<JobId>),

  /// Show the path containing log files.
  #[command(long = "get_logdir")]
  GetLogDir,

  /// Set the path containing log files to the specified path.
  #[command(
    required = false,
    required = false,
    long = "set_logdir",
    value_name = "path"
  )]
  SetLogDir(Option<PathBuf>),

  /// Show the last ten lines of the output file of the named job, or the last running/run if not specified. If the job is still running, it will keep on showing the additional output until the job finishes. On exit, it returns the errorlevel of the job, as in -c.
  #[command(short = 't', value_name = "id")]
  Tail(Option<JobId>),

  /// Run the system's cat to the output file of the named job, or the last running/run if not specified. It will block until all the output can be sent to standard output, and will exit with the job errorlevel as in -c.
  #[command(short = 'c', value_name = "id")]
  Cat(Option<JobId>),

  /// Show the pid of the named job, or the last running/run if not specified.
  #[command(short = 'p', value_name = "id")]
  ShowPid(Option<JobId>),

  /// Show the output file name of the named job, or the last running/run if not specified.
  #[command(short = 'o', value_name = "id")]
  OutputFile(Option<JobId>),

  /// Show the job state of the named job, or the last in the queue.
  #[command(short = 's', value_name = "id")]
  ShowState(Option<JobId>),

  /// Remove the named job, or the last in the queue.
  #[command(short = 'r', value_name = "id")]
  Remove(Option<JobId>),

  /// Wait for the named job, or for the last in the queue.
  #[command(short = 'w', value_name = "id")]
  WaitFor(Option<JobId>),

  /// Kill the process group of the named job (SIGTERM), or the last running/run job if not specified.  Equivalent to kill -- -‘tsp -p‘
  #[command(short = 'k', value_name = "id")]
  Kill(Option<JobId>),

  /// Make the named job (or the last in the queue) urgent - this means that it goes forward in the queue so it can run as soon as possible.
  #[command(short = 'u', value_name = "id")]
  Urgent(Option<JobId>),

  /// Show information about the named job (or the last run). It will show the command line, some times related to the task, and also any information resulting from TS_ENV (Look at ENVIRONMENT).
  #[command(short = 'i', value_name = "id")]
  Info(Option<JobId>),
  //    /// -U <id-id>
  //    /// Interchange the queue positions of the named jobs (separated by a hyphen and no spaces).
  #[command(short = 'U', value_name = "id-id")]
  Interchange(Option<InterchangeS>),
}

#[derive(Args, Clone, Debug, Serialize, Deserialize)]
pub struct GetEnv {
  env: Option<Var>,
}

#[derive(Args, Clone, Debug, Serialize, Deserialize)]
pub struct SetEnv {
  env: Option<Var>,
}

#[derive(Args, Clone, Debug, Serialize, Deserialize)]
pub struct UnsetEnv {
  env: Option<Var>,
}
*/
