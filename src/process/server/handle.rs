use super::Server;
use crate::process::msg::Msg;

impl Server {
  fn handle(&mut self, msg: Msg) {
    use Msg::*;
    match msg {
      KillServer => {
        self.stop();
      }
      NewJob(_new_job) => {
        //self.queue.push(new_job);
      }
      _ => {
        unimplemented!();
      } //NewJobOk(job),
        //RunJob(_) => {} ,
        //RunJobOk(_) => {} ,
        //EndJob(_) => {} ,
        //List(_) => {} ,
        //ListGpu(_) => {} ,
        //ListLine(_) => {} ,
        //ClearFinished(_) => {} ,
        //AskOutput(_) => {} ,
        //AnswerOutput(_) => {} ,
        //Removejob(_) => {} ,
        //RemovejobOk(_) => {} ,
        //Waitjob(_) => {} ,
        //WaitRunningJob(_) => {} ,
        //WaitjobOk(_) => {} ,
        //Urgent(_) => {} ,
        //UrgentOk(_) => {} ,
        //GetState(_) => {} ,
        //AnswerState(_) => {} ,
        //SwapJobs((JobId, JobId)),
        //SwapJobsOk(_) => {} ,
        //Info(_) => {} ,
        //InfoData(_) => {} ,
        //SetMaxSlots(_) => {} ,
        //GetMaxSlots(_) => {} ,
        //GetMaxSlotsOk(_) => {} ,
        //GetVersion(_) => {} ,
        //Version(_) => {} ,
        //NewJobNok(_) => {} ,
        //CountRunning(_) => {} ,
        //GetLabel(_) => {} ,
        //LastId(_) => {} ,
        //KillAll(_) => {} ,
        //GetCmd(_) => {} ,
        //GetEnv(_) => {} ,
        //SetEnv(_) => {} ,
        //UnsetEnv(_) => {} ,
        //SetFreePerc(_) => {} ,
        //GetFreePerc(_) => {} ,
        //GetLogdir(_) => {} ,
        //SetLogdir(_) => {} ,
    }
  }
}
