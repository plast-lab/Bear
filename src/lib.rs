// Copyright (c) 2017 László Nagy
//
// Licensed under the MIT license <LICENSE or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to those terms.

use std::vec::Vec;

#[macro_use]
extern crate serde_derive;

extern crate serde_json;

type Pid = i32;

#[derive(Serialize, Deserialize, Debug)]
struct Execution {
    pid: Pid,
    cwd: String,
    cmd: Vec<String>,
}
