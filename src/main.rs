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

struct HostsFile {
    f: File,
    lines: Vec<String>,
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

        // TODO be smarter when parsing lines
        for line in reader.lines() {
            self.lines.push(line.unwrap());
        }

        Ok(())
    }

    fn block(&mut self, host_list: &Vec<String>) -> io::Result<()> {
        for host in host_list {
            self.lines.push(format!("{}\t{}", LOCALHOST_ADDR, host));
        }

        self.flush()?;
        Ok(())
    }

    fn flush(&mut self) -> io::Result<()> {
        let mut writer = BufWriter::new(&mut self.f);
        writer.seek(SeekFrom::Start(0))?;

        for line in &self.lines {
            writer.write(line.as_bytes())?;
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
        "play" => println!("Let's play!"),
        "work" => {
            hosts_file.block(&hosts_to_block)?;
            println!("Let's work!");
        }
        _ => eprintln!("{}", &USAGE),
    }

    Ok(())
}
