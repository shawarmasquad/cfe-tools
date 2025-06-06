#![allow(unused_imports)]
use std::{fs, process::Command, str};
mod build_core;
mod build_remove;
mod global;
mod help;
mod install_core;
mod utils;
use crate::build_core::build_core;
use crate::build_remove::*;
use crate::global::Config;
use crate::global::read_config;
use crate::help::{print_help, print_version};
use crate::install_core::install_core;
use crate::utils::{export_scripts, gen_fe_includes, run_core};
use std::env::current_exe;

fn main() {
    let cmd: String;
    match std::env::args().nth(1) {
        Some(v) => cmd = v,
        None => {
            println!("ERROR: unexpected argument for command field.");
            return;
        }
    }

    match cmd.as_str() {
        "install-core" => install_core(),
        "build-core" => build_core(),
        "build-remove" => build_remove(),
        "gen-includes" => println!("{}", gen_fe_includes()),
        "run-core" => run_core(),
        "export-scripts" => export_scripts(),
        "--help" => print_help(),
        "-h" => print_help(),
        "--version" => print_version(),
        "-v" => print_version(),
        _ => println!("ERROR: unexpected argument for command field."),
    }
}
