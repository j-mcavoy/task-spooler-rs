use super::Exe;
use std::path::PathBuf;

#[allow(non_snake_case)]
pub struct Env {
  /// Limit the number of job results (finished tasks) you want in the queue. Use this option if
  /// you are tired of -C.
  TS_MAXFINISHED: u32,

  /// The maximum number of tsp server connections to clients. This will make the tsp clients block
  /// until connections are freed. This helps, for example, on systems with a limited number of
  /// processes, because each job waiting in the queue remains as a process. This
  TS_MAXCONN: u32,

  /// If the variable exists pointing to an executable, it will be run by the client
  /// after the queued job. It uses execlp, so PATH is used if there are no slashes in the variable
  /// content.
  ///
  /// The executable is run with four parameters: jobid errorlevel  output_filename and
  /// command.
  TS_ONFINISH: Option<Exe>,

  /// As the program output and the unix socket are thought to be stored in a temporary directory, TMPDIR will be used if defined, or /tmp otherwise.
  TMPDIR: PathBuf,

  /// Each  queue has a related unix socket. You can specify the socket path with this environment
  /// variable. This way, you can have a queue for your heavy disk operations, another for heavy
  /// use of ram., and have a simple script/alias wrapper over tsp for those special queues. If it
  /// is not specified, it will be $TMPDIR/socket-ts.[uid].
  TS_SOCKET: PathBuf,

  /// Set the number of slots at the start of the server, similar to -S, but the contents of the
  /// variable are read only when running the first instance of tsp.
  TS_SLOTS: u32,
  /// Send the letters with job results to the address specified in this variable.
  /// Otherwise, they are sent to $USER or if not defined, nobody.  The system
  /// /usr/sbin/sendmail is used. The job outputs are not sent as an attachment, so
  /// understand the consequences if you use the -gm flags together.
  TS_MAILTO: String,

  /// As seen above, it is used for the mail destination if TS_MAILTO is not specified.:
  USER: String,

  /// If it is defined when starting the queue server (probably the first tsp command run), on
  /// SIGTERM the queue status will be saved to the file pointed by this environment variable
  /// - for example, at system shutdown.
  TS_SAVELIST: Option<PathBuf>,

  /// This has a command to be run at enqueue time through /bin/sh. The output of the command
  /// will be readable through the option -i. You can use a command which shows relevant
  /// environment for the command run.  For example, you may use TS_ENV='pwd;set;mount'.
  TS_ENV: Option<Exe>,
}

impl Default for Env {
  fn default() -> Self {
    let tmp_dir = "/tmp".into();
    let uid = users::get_current_uid().to_string();

    Self {
      TS_MAXFINISHED: 100,
      TS_MAXCONN: 10,
      TS_ONFINISH: None,
      TMPDIR: tmp_dir,
      TS_SOCKET: "".into(),
      TS_SLOTS: 1,
      TS_MAILTO: uid.clone(),
      USER: uid,
      TS_SAVELIST: None,
      TS_ENV: None,
    }
  }
}
