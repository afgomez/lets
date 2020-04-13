use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

const USAGE: &'static str = "
lets - A simple distraction blocker

Blocks hosts specified on a file. It needs write access to /etc/hosts

Usage:
    $ sudo lets play
    $ sudo lets work

Configuration:
    To block hosts, edit `/etc/hosts.block` and add one host per line.
";

const HOST_LIST_FILE: &'static str = "/etc/hosts.block";

fn load_hosts_list(fpath: &Path) -> io::Result<Vec<String>> {
    let f = File::open(fpath)?;
    let reader = BufReader::new(f);
    let mut hosts: Vec<String> = Vec::new();

    // TODO:
    // - Allow comments
    // - Ignore empty lines
    for line in reader.lines() {
        hosts.push(line.unwrap());
    }

    Ok(hosts)
}

fn main() -> io::Result<()> {
    let arg = std::env::args().nth(1);
    let action = arg.unwrap_or(String::from(""));

    let _hosts_to_block = load_hosts_list(Path::new(&HOST_LIST_FILE))?;

    match action.as_str() {
        "play" => println!("Let's play!"),
        "work" => println!("Let's work!"),
        _ => eprintln!("{}", &USAGE),
    }

    Ok(())
}
