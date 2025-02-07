use super::msg;
use std::time::{Duration, SystemTime};

pub type JobId = u64;

pub struct Job {
  next: Box<Option<Job>>,
  jobid: i32,
  command: String,
  state: JobState,
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
  num_gpus: i32,
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
  Finished,
  Skipped,
  HoldingClient,
}

pub enum ExitCodes {
  Ok = 0,
  UnknownError = -1,
  QueueFull = 2,
}
