// Copyright © Jordan Singh 2022

mod daemon;
use daemon::Daemon;

fn main() {
    Daemon::run_for_thread();
}
