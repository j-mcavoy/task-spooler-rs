use super::job::*;
use crate::cli::Var;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum Msg {
  AnswerOutput(Option<JobId>),
  AnswerState(Option<JobId>),
  AskOutput(Option<JobId>),
  ClearFinished,
  CountRunning,
  EndJob(Option<JobId>),
  GetCmd(Option<JobId>),
  GetEnv(Var),
  GetFreePerc(Option<JobId>),
  GetLabel(Option<JobId>),
  GetLogdir,
  GetMaxSlots(Option<JobId>),
  GetMaxSlotsOk(Option<JobId>),
  GetState(Option<JobId>),
  GetVersion(Option<JobId>),
  Info(Option<JobId>),
  InfoData(Option<JobId>),
  KillAll,
  KillServer,
  LastId,
  List(ListFormat),
  ListGpu(Option<JobId>),
  Tail(Option<JobId>),
  NewJob(NewJob),
  NewJobNok(Option<JobId>),
  NewJobOk(NewJob),
  Removejob(Option<JobId>),
  RemovejobOk(Option<JobId>),
  RunJob(Option<JobId>),
  RunJobOk(Option<JobId>),
  SetEnv(Var, Var),
  SetFreePerc(Option<JobId>),
  SetLogdir(PathBuf),
  SetMaxSlots(Option<JobId>),
  SwapJobs((JobId, JobId)),
  SwapJobsOk(Option<JobId>),
  UnsetEnv(Var),
  Urgent(Option<JobId>),
  UrgentOk(Option<JobId>),
  Version(Option<JobId>),
  WaitRunningJob(Option<JobId>),
  Waitjob(Option<JobId>),
  WaitjobOk(Option<JobId>),
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

#[derive(PartialEq, Default, Debug, Serialize, Deserialize)]
pub struct NewJob {
  command_size: i32,
  store_output: bool,
  should_keep_finished: bool,
  label_size: i32,
  env_size: i32,
  depend_on_size: i32,
  wait_enqueuing: bool,
  num_slots: usize,
}
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Output {
  ofilename_size: usize,
  store_output: bool,
  pid: u16,
}
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Result {
  errorlevel: u8,
  died_by_signal: bool,
  signal: i32,
  user_ms: f32,
  system_ms: f32,
  real_ms: f32,
  skipped: i32,
}

#[derive(PartialEq, Default, Debug, Clone, ValueEnum, Serialize, Deserialize)]
pub enum ListFormat {
  #[default]
  Default,
  Tab,
  Json,
}
