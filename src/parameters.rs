// Copyright (c) 2017 László Nagy
//
// Licensed under the MIT license <LICENSE or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to those terms.

use std::env;
use std::process;
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Parameters {
    pub cc: String,
    pub cxx: String,
}

const ENV_KEY: &'static str = "__BEAR";

pub fn read() -> Option<Parameters> {
    let value = env::var(ENV_KEY).unwrap();
    read_from_string(value)
}

pub fn write(config: Parameters, command: &mut process::Command) -> &mut process::Command {
    let value = write_to_string(config);
    match value {
        Some(value) => command.env(ENV_KEY, value),
        None => command,
    }
}

fn read_from_string(input: String) -> Option<Parameters> {
    let result = serde_json::from_str(input.as_str());
    match result {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}

fn write_to_string(input: Parameters) -> Option<String> {
    let result = serde_json::to_string(&input);
    match result {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}
