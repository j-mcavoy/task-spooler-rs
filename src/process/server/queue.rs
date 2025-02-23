use crate::process::{Job, JobId};
use std::collections::VecDeque;

#[derive(Default)]
pub struct Queue {
  jobs: VecDeque<Job>,
}

impl Queue {
  pub fn new() -> Self {
    Self::default()
  }
  pub fn push(&mut self, job: Job) {
    self.jobs.push_back(job);
  }
}
