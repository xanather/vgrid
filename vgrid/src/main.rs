// Copyright © Jordan Singh 2022
#![windows_subsystem = "windows"]

mod daemon;
use daemon::Daemon;

fn main() {
    Daemon::run_for_thread();
}
