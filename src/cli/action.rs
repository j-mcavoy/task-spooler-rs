use crate::process::{job::JobId, msg::*};
use clap::*;
use std::path::PathBuf;

type Var = String;

#[allow(non_snake_case)]
#[derive(Args, Clone)]
#[group(required = false, multiple = false)]
pub struct Opt {
  /// When adding a job to tsp, we can specify how it will be run and how the results will be collected:

  /// Do not store the standard output/error in a file at $TMPDIR - let it use the file descriptors decided by the calling process. If it is not used, the jobid for the new task will be output to stdout.
  #[arg(short = 'n')]
  StoreOutput: bool,
  /// Pass the output through gzip (only if -n ). Note that the output files will not have a .gz extension.
  #[arg(short = 'z')]
  GZip: bool,
  /// Do not put the task into the background. Wait for the command to run without detaching from the terminal. The exit code will be that of the command, and if used together with -n, no result will be stored in the queue.
  #[arg(short = 'f')]
  Follow: bool,
  /// Mail the results of the command (output and exit code) to $TS_MAILTO , or to the $USER using /usr/sbin/sendmail.  Look at ENVIRONMENT.
  #[arg(short = 'm')]
  Mail: bool,
  /// Add a label to the task, which will appear next to its command when listing the queue. It makes more comfortable distinguishing similar commands with different goals.
  #[arg(short = 'L', value_name = "label")]
  Label: Var,

  /// Run the command only after the last command finished.  It does not depend on how its dependency ends.
  #[arg(short = 'd')]
  Dependent: bool,

  /// Run the command only after the specified job IDs finished.  It does not depend on how its dependencies end.
  #[arg(short = 'D', value_name = "id,...")]
  Dependencies: Vec<JobId>,

  /// Run the command only if the job of given id finished well (errorlevel = 0). This
  /// new task enqueued depends on the result of the previous command. If the task is not
  /// run, it is considered as failed for further dependencies.  If the server doesn't have
  /// the  job id in its list, it will be considered as if the job failed.
  #[arg(short = 'W', value_name = "id,...")]
  DependenciesWell: Vec<JobId>,

  /// In the case the queue is full (due to TS_MAXCONN or system limits), by default tsp will block the enqueuing command. Using -B, if the queue is full it will exit returning the value 2 instead of blocking.
  #[arg(short = 'b')]
  Blocking: bool,

  /// Keep  two  different  output files for the command stdout and stderr. stdout goes to the file announced by tsp (look at -o), and stderr goes to the stdout file with an additional ".e". For example, /tmp/ts-out.SKsDw8 and /tmp/ts-out.SKsDw8.e.  Only the stdout
  /// file gets created with mkstemp, ensuring it does not overwrite any other; the ".e" will be overwritten if it existed.
  #[arg(short = 'E')]
  EOutput: bool,

  /// Set the log name to the specified name. Do not include any path in the specified name.
  #[arg(short = 'O', value_name = "name")]
  Output: Var,

  /// Run the command only if there are num slots free in the queue. Without it, the job will run if there is one slot free. For example, if you use the queue to feed cpu cores, and you know that a job will take two cores, with -N you can let tsp know that.
  #[arg(short = 'N', value_name = "num")]
  Num: usize,
}

#[allow(non_snake_case)]
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
  #[arg(short = 'u', value_name = "id")]
  Urgent: Option<JobId>,

  /// Show information about the named job (or the last run). It will show the command line, some times related to the task, and also any information resulting from TS_ENV (Look at ENVIRONMENT).
  #[arg(short = 'i', value_name = "id")]
  Info: Option<JobId>,
  //    /// -U <id-id>
  //    /// Interchange the queue positions of the named jobs (separated by a hyphen and no spaces).
  #[arg(short = 'U', value_name = "id-id")]
  Interchange: Option<InterchangeS>,
}

#[derive(Clone)]
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
