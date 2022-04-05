#![feature(never_type)]

use std::sync::mpsc::Sender;

use runbun::{daemons::{Daemon, DaemonConfig}, action::Action};

fn spawner(tx: Sender<Action>, daemons: &mut Vec<Daemon>, configs: &[DaemonConfig]) {
    for config in configs {
        match Daemon::new(config.clone(), tx.clone()) {
            Ok(d) => {
                daemons.push(d);
            }
            Err(e) => {
                tx.send(Action::DaemonConfigWarning(e.to_string())).ok();
            }
        }
            }
        }

fn main() -> Result<!, runbun::errors::Error> {
    message();
    let (tx, rx) = std::sync::mpsc::channel();
    let mut daemons: Vec<Daemon> = Vec::new();
    let txc = tx.clone();
    let daemon_configs = runbun::daemons::load_all(txc)?;

    let txc = tx.clone();
    spawner(txc, &mut daemons, &daemon_configs);

    while let Ok(next) = rx.recv() {
        match next {
            Action::DaemonStarted(name) => {
                println!("OK:\t\t{}", name)
            }
            Action::DaemonDied(name) => {
                println!("FAILED:\t{}", name)
            }
            Action::DaemonComplete(name) => {
                println!("COMPLETE:\t{}", name)
            }
            Action::DaemonConfigWarning(info) => {
                eprintln!("WARNING:\t{}", info)
            }
        }
    }

    panic!("The bun stopped running...");
}

fn listener(tx: Sender<Action>) {
    
}

fn message() {
    use std::io::{stdout, Write};
    let mut oup = stdout();
    oup.write(format!("runbun {}\n", env!("CARGO_PKG_VERSION")).as_bytes())
        .ok();
    oup.write(b"run bun run...\n").ok();
    oup.flush().ok();
}
