use runbun::{daemons, action::Action};

fn main() {
    let mut valid: Vec<String> = Vec::new();
    let (tx, rx) = std::sync::mpsc::channel();

    if let Ok(dcs) = daemons::load_all(tx) {
        for d in dcs {
            valid.push(d.command.display().to_string());
        }
    }

    for msg in rx {
        match msg {
            Action::DaemonConfigWarning(info) => {
                eprintln!("WARNING:\t{}", info)
            }
            Action::DaemonStarted(name) => {
                println!("OK:\t\t{}", name)
            }
            Action::DaemonDied(name) => {
                println!("FAILED:\t{}", name)
            }
            Action::DaemonComplete(name) => {
                println!("COMPLETE:\t{}", name)
            }
        }
    }

    println!("{} valid configs\n", valid.len());

    if valid.len() == 0 {
        return;
    }

    print!("{}", valid.remove(0));
    for c in valid {
        print!(", {}", c)
    }
    println!();
}
