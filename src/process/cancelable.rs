use std::sync::{atomic::AtomicBool, Arc};

pub enum LoopState {
  Continue,
  Break,
}

pub trait Cancellable {
  type Error;

  fn for_each(&mut self) -> Result<LoopState, Self::Error>;

  fn run(&mut self) -> Result<(), Self::Error> {
    use LoopState::*;
    loop {
      match self.for_each() {
        Ok(Continue) => Ok(()),
        Ok(Break) => break Ok(()),
        e => return e,
      }
    }
  }

  fn spwan(mut self) -> Handle<Self::Error>
  where
    Self: Sized + Send + 'static,
    Self::Error: Send + 'static,
  {
    use LoopState::*;

    let keep_running = Arc::new(AtomicBool::new(true));

    let jh = {
      let keep_running = keep_running.clone();
      std::thread::spawn(move || {
        while keep_running.load(SeqCst) {
          match self.for_each() {
            Ok(Continue) => {}
            _ => break,
          }
        }
        Ok(())
      })
    };

    Handle {
      keep_running,
      executor: jh,
    }
  }
}

use std::thread::JoinHandle;

pub struct Handle<E> {
  keep_running: Arc<AtomicBool>,
  executor: JoinHandle<Result<(), E>>,
}
impl<E> Handle<E> {
  pub fn terminate(&self) {
    self.keep_running.store(false, SeqCst);
  }
  pub fn canceler(&self) -> Canceller {
    Canceller {
      keep_running: self.keep_running.clone(),
    }
  }

  pub fn wait(self) -> Result<(), E> {
    match self.executor.join() {
      Ok(r) => return r,
      Err(e) => panic!("{:?}", e),
    }
  }
}

use std::sync::atomic::Ordering::SeqCst;
struct Canceller {
  keep_running: Arc<AtomicBool>,
}
impl Canceller {
  pub fn cancel(&self) {
    self.keep_running.store(false, SeqCst);
  }
}
