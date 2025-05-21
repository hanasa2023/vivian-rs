pub mod api;
pub mod client;
pub mod error;
pub mod types;

pub use client::MilkyClient;
pub use error::{MilkyError, Result};
pub use types::{
    event::{Event, EventKind},
    friend::{Friend, FriendCategory},
    group::{Group, GroupAnnouncement, GroupFile, GroupFolder, GroupMember},
    message::{in_coming, out_going},
};
