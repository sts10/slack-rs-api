// Copyright 2015-2016 the slack-rs authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Low-level, direct interface for the [Slack Web
//! API](https://api.slack.com/methods).
#[macro_use]
extern crate derive_error;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_qs;
extern crate reqwest;

extern crate types;
#[macro_use]
pub extern crate requests;

pub use types::*;
use requests::*;

mod mods;
pub use mods::*;

pub extern crate channels;
pub extern crate api;
