mod action;
use action::*;
use clap::*;

#[derive(Parser)]
#[command(version, about)]
#[clap(trailing_var_arg = true)]
struct Cli {
  /// Actions
  ///
  /// Actions:  [-KClhVTRqM]  [-t  [id]]  [-c  [id]]  [-p [id]] [-o [id]] [-s [id]] [-r [id]] [-w [id]] [-k [id]] [-u [id]] [-i [id]] [-U <id-id>] [-S [num]] [-a/--get_label [id]] [-F/--full_cmd [id]] [--getenv [var]] [--setenv [var=val]] [--unsetenv [var]] [--get_logdir]
  #[command(flatten)]
  action: Action,

  /// executable command to queue
  #[arg(trailing_var_arg = true, allow_hyphen_values = true, group = "input")]
  command: Option<Vec<String>>,
}

pub fn cli() {
  let cli = Cli::parse();

  //    use action::Action::*;
  //    if let Some(action) = cli.action {
  //        match action {
  //            GetEnv { var } => {
  //                let args = vec!["--getenv", var.as_str()];
  //                crate::ffi::tsp_c(args);
  //            }
  //            SetEnv { var } => {
  //                let args = vec!["--setenv", var.as_str()];
  //                crate::ffi::tsp_c(args);
  //            }
  //            UnsetEnv { var } => {
  //                let args = vec!["--unsetenv", var.as_str()];
  //                crate::ffi::tsp_c(args);
  //            }
  //            KillAll => {
  //                let args = vec!["-K"];
  //                crate::ffi::tsp_c(args);
  //            }
  //            Sigterm => {
  //                let args = vec!["-T"];
  //                crate::ffi::tsp_c(args);
  //            }
  //            Clear => {
  //                let args = vec!["-C"];
  //                crate::ffi::tsp_c(args);
  //            }
  //            List => {}
  //            Serialize { format } => {}
  //            LastQueue => {}
  //            CountRunning => {}
  //            GetLabel { id } => {}
  //            FullCmd { id } => {}
  //            GetLogDir => {}
  //            SetLogDir { path } => {}
  //            Tail { id } => {}
  //            Cat { id } => {}
  //            ShowPid { id } => {}
  //            OutputFile { id } => {}
  //            ShowState { id } => {}
  //            Remove { id } => {}
  //            WaitFor { id } => {}
  //            KillAll => {}
  //            Urgent { id } => {}
  //            Info { id } => {}
  //            Interchange { rhs, lhs } => {}
  //            _ => {}
  //        };
  //    }
}

//struct CommandLine {
//    request:Request,
//    need_server: bool,
//    store_output: bool,
//    stderr_apart: bool,
//    should_go_background:bool,
//    should_keep_finished:bool,
//    send_output_by_mail:bool,
//    gzip:bool,
//    depend_on: Vec<i32>, /* -1 means depend on previous */
//    //depend_on_size; uneeded
//    max_slots: usize, /* How many jobs to run at once */
//    jobid: job::JobId, /* When queuing a job, main.c will fill it automatically from the server
//    answer to NEWJOB */
//    int jobid2;
//    int wait_enqueuing;
//    struct {
//        char **array;
//        int num;
//    } command;
//    char *label;
//    int num_slots; /* Slots for the job to use. Default 1 */
//    int require_elevel;  /* whether requires error level of dependencies or not */
//    int gpus;
//    int *gpu_nums;
//    int wait_free_gpus;
//    char *logfile;
//    enum ListFormat list_format;
//};
