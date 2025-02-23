use super::msg;
use std::{
  sync::RwLock,
  time::{Duration, SystemTime},
};

pub type JobId = u16;

pub struct Job {
  jobid: JobId,
  command: std::process::Command,
  state: RwLock<JobState>,
  result: msg::Result,
  output_filename: String,
  store_output: i32,
  pid: i32,
  should_keep_finished: i32,
  depend_on: Vec<JobId>,
  depend_on_size: i32,
  notify_errorlevel_to: Vec<JobId>,
  notify_errorlevel_to_size: i32,
  dependency_errorlevel: i32,
  label: String,
  info: Procinfo,
  num_slots: i32,
}
pub struct Procinfo {
  ptr: String,
  nchars: usize,
  allocchars: usize,
  enqueue_time: Duration,
  start_time: SystemTime,
  end_time: SystemTime,
}

#[derive(Default)]
pub enum JobState {
  #[default]
  Queued,
  Allocating,
  Running,
  Finished(ExitCode),
  Skipped,
  HoldingClient,
}

pub enum ExitCode {
  Ok = 0,
  UnknownError = -1,
  QueueFull = 2,
}
