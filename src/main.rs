const USAGE: &'static str = "
lets - A simple distraction blocker

Blocks hosts specified on a file. It needs write access to /etc/hosts

Usage:
    $ sudo lets play
    $ sudo lets work

Configuration:
    To block hosts, edit `/etc/hosts.block` and add one host per line.
";

fn main() {
    let arg = std::env::args().nth(1);
    let action = arg.unwrap_or(String::from(""));

    match action.as_str() {
        "play" => println!("Let's play!"),
        "work" => println!("Let's work!"),
        _ => eprintln!("{}", &USAGE),
    }
}
