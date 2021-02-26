//! Parameters of
//! [window messages](https://docs.microsoft.com/en-us/windows/win32/winmsg/about-messages-and-message-queues).
//!
//! Each message struct defines the parameters it receives and also its return
//! type.

#[macro_use]
mod macros;

mod message;
mod wm_structs_bm;
mod wm_structs_cb;
mod wm_structs_dtm;
mod wm_structs_hdm;
mod wm_structs_lb;
mod wm_structs_lvm;
mod wm_structs_sb;
mod wm_structs_stm;
mod wm_structs;

pub use message::{Message, MessageHandleable};
pub use wm_structs_bm::*;
pub use wm_structs_cb::*;
pub use wm_structs_dtm::*;
pub use wm_structs_hdm::*;
pub use wm_structs_lb::*;
pub use wm_structs_lvm::*;
pub use wm_structs_sb::*;
pub use wm_structs_stm::*;
pub use wm_structs::*;