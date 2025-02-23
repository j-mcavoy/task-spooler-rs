//pub mod cancelable;
pub mod client;
pub mod job;
pub mod msg;
pub mod request;
pub mod server;
pub use job::*;

pub enum ProcessType {
  Client,
  Server,
}

pub static SOCK: &str = "seqpacket_listener.socket";
