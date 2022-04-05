use serde::Deserialize;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::mpsc::Sender;
use std::thread;
use std::thread::JoinHandle;

use crate::action::Action;
use crate::errors::Error;

pub struct Daemon {
    child: Option<JoinHandle<()>>,
    config: DaemonConfig,
}

impl Daemon {
    pub fn new(config: DaemonConfig, tx: Sender<Action>) -> Result<Self, Error> {
        let mut c = Daemon {
            child: None,
            config,
        };
        c.start(tx)?;
        Ok(c)
    }

    pub fn is_oneshot(&self) -> bool {
        self.config.is_oneshot()
    }

    pub fn start(&mut self, tx: Sender<Action>) -> Result<(), Error> {
        let config = self.config.clone();
        let child = thread::spawn(move || crate::daemon_holder(tx, config));
        self.child = Some(child);
        Ok(())
    }

    pub fn name(&self) -> String {
        self.config.command.display().to_string()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DaemonConfig {
    pub desc: String,
    pub command: PathBuf,
    pub args: Vec<String>,
    pub env: Vec<Envar>,
    pub user: Option<u64>,
    pub run_group: u8,
    pub kind: DaemonKind,
}

impl DaemonConfig {
    pub fn is_oneshot(&self) -> bool {
        self.kind == DaemonKind::Oneshot
    }

    pub fn is_daemon(&self) -> bool {
        self.kind == DaemonKind::Daemon
    }
}

impl PartialOrd for DaemonConfig {
    fn partial_cmp(&self, other: &DaemonConfig) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DaemonConfig {
    fn cmp(&self, other: &DaemonConfig) -> std::cmp::Ordering {
        self.command.cmp(&other.command)
    }
}

impl PartialEq for DaemonConfig {
    fn eq(&self, other: &DaemonConfig) -> bool {
        self.command == other.command && self.run_group == other.run_group
    }
}

impl Eq for DaemonConfig {}

pub fn load_all(tx: Sender<Action>) -> Result<Vec<DaemonConfig>, Error> {
    let mut daemon_configs: Vec<DaemonConfig> = Vec::new();

    let mut config_path = PathBuf::from("/etc/runbun.d/");

    while !config_path.exists() {
        const EXT_PATH: &str = "/usr/local/etc/runbub.d/";
        if !config_path.exists() && config_path.display().to_string() != EXT_PATH {
            config_path = PathBuf::from("/usr/local/etc/runbun.d/");
            continue;
        }
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "No daemon config directory found",
        ))?;
    }

    for entry in std::fs::read_dir(config_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            match load_config(&path) {
                Ok(config) => {
                    daemon_configs.push(config);
                }
                Err(err) => {
                    tx.send(Action::DaemonConfigWarning(format!("{:?} parsing {}", err, path.display()))).ok();
                }
            }
        }
    }

    daemon_configs.sort();

    Ok(daemon_configs)
}

fn load_config(path: &Path) -> Result<DaemonConfig, Error> {
    let mut config_file = std::fs::File::open(path)?;
    let mut config_string = String::new();
    config_file.read_to_string(&mut config_string)?;
    let config: DaemonConfig = toml::from_str(&config_string)?;
    Ok(config)
}

#[derive(Debug, Clone, Deserialize)]
pub struct Envar {
    name: String,
    value: String,
}

impl Envar {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn compile(self) -> (String, String) {
        (self.name, self.value)
    }
}

impl FromStr for Envar {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.split('=').collect::<Vec<&str>>();

        if s.len() != 2 {
            return Err(Error::InvalidEnvar(s.join("=")));
        }

        Ok(Envar {
            name: s[0].to_string(),
            value: s[1].to_string(),
        })
    }
}

#[derive(Debug, Copy, Clone, Deserialize, PartialEq, Eq)]
pub enum DaemonKind {
    Oneshot,
    Daemon,
}

impl FromStr for DaemonKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "oneshot" => Ok(DaemonKind::Oneshot),
            "daemon" => Ok(DaemonKind::Daemon),
            _ => Err(Error::InvalidDaemonKind(s.to_string())),
        }
    }
}
