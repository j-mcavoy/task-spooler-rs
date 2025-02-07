pub mod client;
pub mod job;
pub mod msg;
pub mod request;
pub mod server;
pub use client::*;
pub use job::*;
pub use msg::*;
pub use request::*;
pub use server::*;

pub enum ProcessType {
  Client,
  Server,
}

pub static SOCK: &str = "seqpacket_listener.socket";
