use regex::Regex;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter, SeekFrom};
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

const LOCALHOST_ADDR: &'static str = "127.0.0.1";
const HOSTS_FILE_PATH: &'static str = "/etc/hosts";
const HOST_LIST_FILE: &'static str = "/etc/hosts.block";

#[derive(Debug)]
struct HostEntry {
    address: String,
    name: String,
}

#[derive(Debug)]
enum HostLine {
    Literal(String),
    Entry(HostEntry),
}

impl HostLine {
    fn to_string(&self) -> String {
        match &self {
            HostLine::Literal(string) => string.clone(),
            HostLine::Entry(host) => format!("{}\t{}", host.address, host.name),
        }
    }
}

struct HostsFile {
    f: File,
    lines: Vec<HostLine>,
}

impl HostsFile {
    fn open(path: &Path) -> io::Result<Self> {
        let f = OpenOptions::new().write(true).read(true).open(&path)?;
        Ok(HostsFile {
            f,
            lines: Vec::new(),
        })
    }

    fn load(&mut self) -> io::Result<()> {
        let reader = BufReader::new(&self.f);
        let re = Regex::new(r"^\s*(\d+\.\d+\.\d+\.\d+)\s*(.?*)\s*$").unwrap();

        for l in reader.lines() {
            let line = l.unwrap();
            let host_line = match re.captures(&line) {
                None => HostLine::Literal(line),
                Some(matched_line) => HostLine::Entry(HostEntry {
                    address: String::from(matched_line.get(1).unwrap().as_str()),
                    name: String::from(matched_line.get(2).unwrap().as_str()),
                }),
            };
            self.lines.push(host_line);
        }

        Ok(())
    }

    fn block(&mut self, host_list: &Vec<String>) -> io::Result<()> {
        for host in host_list {
            self.lines.push(HostLine::Entry(HostEntry {
                address: String::from(LOCALHOST_ADDR),
                name: String::from(host),
            }));
        }

        self.flush()?;
        Ok(())
    }

    fn flush(&mut self) -> io::Result<()> {
        let mut writer = BufWriter::new(&mut self.f);
        writer.seek(SeekFrom::Start(0))?;

        for line in &self.lines {
            writer.write(line.to_string().as_bytes())?;
            writer.write(b"\n")?;
        }

        Ok(())
    }
}

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

    let hosts_to_block = load_hosts_list(Path::new(&HOST_LIST_FILE))?;
    let mut hosts_file = HostsFile::open(Path::new(&HOSTS_FILE_PATH))?;
    hosts_file.load()?;

    match action.as_str() {
        "play" => {
            // hosts_file.unblock(&hosts_to_block)?;
            println!("Let's play!")
        }
        "work" => {
            hosts_file.block(&hosts_to_block)?;
            println!("Let's work!");
        }
        _ => eprintln!("{}", &USAGE),
    }

    Ok(())
}
