// Copyright 2025 hanasaki <hanasakayui2022@gmail.com>. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub mod api;
pub mod client;
pub mod error;
pub mod logger;
pub mod types;

pub use client::MilkyClient;
pub use error::{MilkyError, Result};
pub use types::{
    event::{Event, EventKind},
    friend::{Friend, FriendCategory},
    group::{Group, GroupAnnouncement, GroupFile, GroupFolder, GroupMember},
    message::{in_coming, out_going},
};
