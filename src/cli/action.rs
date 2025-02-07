use crate::process::{job::JobId, msg::*};
use clap::*;
use std::path::PathBuf;

type Var = String;

#[derive(Args)]
#[group(required = false, multiple = false)]
pub struct Action {
  /// Get the specified environment variable value from the tsp server.
  #[arg(long = "getenv", value_name = "ENV")]
  GetEnv: Var,

  /// Get the specified environment variable value from the tsp server.
  #[arg(long = "setenv", value_name = "ENV")]
  SetEnv: Var,

  /// Remove the specified environment variable from the tsp server. Set the specified environment variable to the tsp server.
  #[arg(long = "unsetenv", value_name = "ENV")]
  UnsetEnv: Var,

  /// Kill the tsp server for the calling client. This will remove the unix socket and all the tsp processes related to the queue. This will not kill the command being run at that time.
  ///
  /// It is not reliable to think that tsp -K will finish when the server is really killed. By now it is a race condition.
  #[arg(short = 'K')]
  KillAll: bool,

  /// Send SIGTERM to all running job groups.
  #[arg(short = 'T')]
  Sigterm: bool,

  /// Clear the results of finished jobs from the queue.
  #[arg(short = 'C')]
  Clear: bool,

  /// Show the list of jobs - to be run, running and finished - for the current queue.  This is the default behaviour if tsp is called without options.
  #[arg(short = 'l')]
  List: bool,

  /// Serialize the job list to the specified format. Choices: {default, json, tab}.
  #[arg(value_enum, long = "serialize", short = 'M')]
  Serialize: ListFormat,

  /// Show the job ID of the last added.
  #[arg(long = "last_queue_id", short = 'q')]
  LastQueue: bool,

  /// Return the number of running jobs
  #[arg(long = "count_running", short = 'R')]
  CountRunning: bool,

  /// Show the job label. Of the last added, if not specified.
  #[arg(long = "get_label", short = 'a', value_name = "id")]
  GetLabel: Option<JobId>,

  /// Show the full command. Of the last added, if not specified.
  #[arg(long = "full_cmd", short = 'F', value_name = "id")]
  FullCmd: Option<JobId>,

  /// Show the path containing log files.
  #[arg(long = "get_logdir")]
  GetLogDir: bool,

  /// Set the path containing log files to the specified path.
  #[arg(long = "set_logdir", value_name = "path")]
  SetLogDir: PathBuf,

  /// Show the last ten lines of the output file of the named job, or the last running/run if not specified. If the job is still running, it will keep on showing the additional output until the job finishes. On exit, it returns the errorlevel of the job, as in -c.
  #[arg(short = 't', value_name = "id")]
  Tail: Option<JobId>,

  /// Run the system's cat to the output file of the named job, or the last running/run if not specified. It will block until all the output can be sent to standard output, and will exit with the job errorlevel as in -c.
  #[arg(short = 'c', value_name = "id")]
  Cat: Option<JobId>,

  /// Show the pid of the named job, or the last running/run if not specified.
  #[arg(short = 'p', value_name = "id")]
  ShowPid: Option<JobId>,

  /// Show the output file name of the named job, or the last running/run if not specified.
  #[arg(short = 'o', value_name = "id")]
  OutputFile: Option<JobId>,

  /// Show the job state of the named job, or the last in the queue.
  #[arg(short = 's', value_name = "id")]
  ShowState: Option<JobId>,

  /// Remove the named job, or the last in the queue.
  #[arg(short = 'r', value_name = "id")]
  Remove: Option<JobId>,

  /// Wait for the named job, or for the last in the queue.
  #[arg(short = 'w', value_name = "id")]
  WaitFor: Option<JobId>,

  /// Kill the process group of the named job (SIGTERM), or the last running/run job if not specified.  Equivalent to kill -- -‘tsp -p‘
  #[arg(short = 'k', value_name = "id")]
  Kill: Option<JobId>,

  /// Make the named job (or the last in the queue) urgent - this means that it goes forward in the queue so it can run as soon as possible.
  #[arg(short = 'U', value_name = "id")]
  Urgent: Option<JobId>,

  /// Show information about the named job (or the last run). It will show the command line, some times related to the task, and also any information resulting from TS_ENV (Look at ENVIRONMENT).
  #[arg(short = 'i', value_name = "id")]
  Info: Option<JobId>,
  //    /// -U <id-id>
  //    /// Interchange the queue positions of the named jobs (separated by a hyphen and no spaces).
  //    #[arg(short = "U" ,value_name = "id")]
  //    Interchange: InterchangeS,
}

#[derive(Clone)]
struct InterchangeS(JobId, JobId);
