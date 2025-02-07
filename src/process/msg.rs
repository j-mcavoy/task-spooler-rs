use super::job::*;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Msg {
  KillServer,
  NewJob(NewJob),
  NewJobOk(NewJob),
  RunJob(Option<JobId>),
  RunJobOk(Option<JobId>),
  EndJob(Option<JobId>),
  List(Option<Option<JobId>>),
  ListGpu(Option<JobId>),
  ListLine(Option<JobId>),
  ClearFinished(Option<JobId>),
  AskOutput(Option<JobId>),
  AnswerOutput(Option<JobId>),
  Removejob(Option<JobId>),
  RemovejobOk(Option<JobId>),
  Waitjob(Option<JobId>),
  WaitRunningJob(Option<JobId>),
  WaitjobOk(Option<JobId>),
  Urgent(Option<JobId>),
  UrgentOk(Option<JobId>),
  GetState(Option<JobId>),
  AnswerState(Option<JobId>),
  SwapJobs(Option<JobId>),
  SwapJobsOk(Option<JobId>),
  Info(Option<JobId>),
  InfoData(Option<JobId>),
  SetMaxSlots(Option<JobId>),
  GetMaxSlots(Option<JobId>),
  GetMaxSlotsOk(Option<JobId>),
  GetVersion(Option<JobId>),
  Version(Option<JobId>),
  NewJobNok(Option<JobId>),
  CountRunning(Option<JobId>),
  GetLabel(Option<JobId>),
  LastId(Option<JobId>),
  KillAll(Option<JobId>),
  GetCmd(Option<JobId>),
  GetEnv(Option<JobId>),
  SetEnv(Option<JobId>),
  UnsetEnv(Option<JobId>),
  SetFreePerc(Option<JobId>),
  GetFreePerc(Option<JobId>),
  GetLogdir(Option<JobId>),
  SetLogdir(Option<JobId>),
}
/*
#[derive(Default)]
pub struct Msg {
    new_job: NewJob,
    output: Output,
    jobid: Option<JobId>,
    result: Result,
    size: usize,
    state: JobState,
    swap: (Option<JobId>, Option<JobId>),
    last_errorlevel: i32,
    max_slots: usize,
    version: u32,
    count_running: usize,
    label: String,
    term_width: u64,
    list_format: ListFormat,
}
*/

#[derive(Default, Debug, Serialize, Deserialize)]
struct NewJob {
  command_size: i32,
  store_output: i32,
  should_keep_finished: i32,
  label_size: i32,
  env_size: i32,
  depend_on_size: i32,
  wait_enqueuing: i32,
  num_slots: i32,
  gpus: i32,
  wait_free_gpus: i32,
}
#[derive(Default, Debug, Serialize, Deserialize)]
struct Output {
  ofilename_size: usize,
  store_output: i32,
  pid: u32,
}
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Result {
  errorlevel: i32,
  died_by_signal: i32,
  signal: i32,
  user_ms: f32,
  system_ms: f32,
  real_ms: f32,
  skipped: i32,
}

#[derive(Default, Debug, Clone, ValueEnum, Serialize, Deserialize)]
pub enum ListFormat {
  #[default]
  Default,
  Tab,
  Json,
}
