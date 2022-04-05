use std::{process::Command, sync::mpsc::Sender};

use action::Action;
use daemons::DaemonConfig;

pub mod action;
pub mod daemons;
pub mod errors;
pub mod socket;

fn daemon_holder(tx: Sender<Action>, config: DaemonConfig) {
    while config.is_daemon() {
        let compiled_env: Vec<(String, String)> = config.env.clone().into_iter().map(|e| e.compile()).collect();
        match Command::new(&config.command).args(&config.args).envs(compiled_env).spawn() {
            Ok(mut child) => {
                tx.send(Action::DaemonStarted(config.desc.clone())).ok();
                if let Err(e) = child.wait() {
                    tx.send(Action::DaemonDied(format!("{}: {}", config.desc, e))).ok();
                } else {
                    tx.send(Action::DaemonDied(config.desc.clone())).ok();
                }
            }
            Err(e) => {
                tx.send(Action::DaemonDied(format!("{}: {}", config.desc, e))).ok();
            }
        }
    }
    tx.send(Action::DaemonComplete(config.desc.clone())).ok();
}
