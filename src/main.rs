// Copyright (c) 2017 László Nagy
//
// Licensed under the MIT license <LICENSE or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to those terms.

#[macro_use]
extern crate clap;

use std::env;
use clap::{App, Arg};

fn main() {
    let default_cc_compiler = env::var("CC").unwrap_or("cc".to_string());
    let default_cxx_compiler = env::var("CXX").unwrap_or("c++".to_string());

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(Arg::with_name("verbose")
            .long("verbose")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
        .arg(Arg::with_name("output")
            .long("output")
            .short("o")
            .takes_value(true)
            .value_name("file")
            .default_value("compile_commands.json")
            .help("The compilation database file"))
        .arg(Arg::with_name("cc_compiler")
            .long("use-cc")
            .takes_value(true)
            .value_name("compiler")
            .default_value(default_cc_compiler.as_str())
            .help("The C compiler which will be used in compiler wrappers"))
        .arg(Arg::with_name("cxx_compiler")
            .long("use-c++")
            .takes_value(true)
            .value_name("compiler")
            .default_value(default_cxx_compiler.as_str())
            .help("The C++ compiler which will be used in compiler wrappers"))
        .arg(Arg::with_name("relative")
            .long("relative")
            .takes_value(true)
            .value_name("directory")
            .help("Makes the database entries relative to this directory"))
        .arg(Arg::with_name("command")
            .value_name("command")
            .takes_value(true)
            .multiple(true)
            .required(true)
            .help("The build command to intercept"))
        .get_matches();

    let cmds: Vec<_> = matches.values_of("command").unwrap().collect();
    for c in &cmds {
        println!("{}", c);
    }
}
