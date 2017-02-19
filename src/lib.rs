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
