use super::Var;
use crate::process::JobId;
use clap::*;

#[allow(non_snake_case)]
#[derive(Args, Debug, Clone)]
#[group(required = false, multiple = false)]
pub struct Opt {
  /// When adding a job to tsp, we can specify how it will be run and how the results will be collected:

  /// Do not store the standard output/error in a file at $TMPDIR - let it use the file descriptors decided by the calling process. If it is not used, the jobid for the new task will be output to stdout.
  #[arg(required = false, short = 'n')]
  StoreOutput: bool,
  /// Pass the output through gzip (only if -n ). Note that the output files will not have a .gz extension.
  #[arg(required = false, short = 'z')]
  GZip: bool,
  /// Do not put the task into the background. Wait for the command to run without detaching from the terminal. The exit code will be that of the command, and if used together with -n, no result will be stored in the queue.
  #[arg(required = false, short = 'f')]
  Follow: bool,
  /// Mail the results of the command (output and exit code) to $TS_MAILTO , or to the $USER using /usr/sbin/sendmail.  Look at ENVIRONMENT.
  #[arg(required = false, short = 'm')]
  Mail: bool,
  /// Add a label to the task, which will appear next to its command when listing the queue. It makes more comfortable distinguishing similar commands with different goals.
  #[arg(required = false, short = 'L', value_name = "label")]
  Label: Option<Var>,

  /// Run the command only after the last command finished.  It does not depend on how its dependency ends.
  #[arg(required = false, short = 'd')]
  Dependent: bool,

  /// Run the command only after the specified job IDs finished.  It does not depend on how its dependencies end.
  #[arg(required = false, short = 'D', value_name = "id,...")]
  Dependencies: Vec<JobId>,

  /// Run the command only if the job of given id finished well (errorlevel = 0). This
  /// new task enqueued depends on the result of the previous command. If the task is not
  /// run, it is considered as failed for further dependencies.  If the server doesn't have
  /// the  job id in its list, it will be considered as if the job failed.
  #[arg(required = false, short = 'W', value_name = "id,...")]
  DependenciesWell: Vec<JobId>,

  /// In the case the queue is full (due to TS_MAXCONN or system limits), by default tsp will block the enqueuing command. Using -B, if the queue is full it will exit returning the value 2 instead of blocking.
  #[arg(required = false, short = 'b')]
  Blocking: bool,

  /// Keep  two  different  output files for the command stdout and stderr. stdout goes to the file announced by tsp (look at -o), and stderr goes to the stdout file with an additional ".e". For example, /tmp/ts-out.SKsDw8 and /tmp/ts-out.SKsDw8.e.  Only the stdout
  /// file gets created with mkstemp, ensuring it does not overwrite any other; the ".e" will be overwritten if it existed.
  #[arg(required = false, short = 'E')]
  EOutput: bool,

  /// Set the log name to the specified name. Do not include any path in the specified name.
  #[arg(required = false, short = 'O', value_name = "name")]
  Output: Option<Var>,

  /// Run the command only if there are num slots free in the queue. Without it, the job will run if there is one slot free. For example, if you use the queue to feed cpu cores, and you know that a job will take two cores, with -N you can let tsp know that.
  #[arg(required = false, short = 'N', value_name = "num")]
  Num: Option<usize>,
}
