# `lets`: a simple productivity tool

`lets` blocks a list of distracting websites of your choice. It works by
modifying your `/etc/hosts` file and pointing the hosts to the own machine.

Because of this, it needs to be run with `sudo`.

## Usage

To use `lets`, first create a file called `/etc/hosts.block` with the hosts
that you want to block.

    $ echo "facebook.com" >> /etc/hosts.block
    $ echo "twitter.com" >> /etc/hosts.block
    $ echo "reddit.com" >> /etc/hosts.block

To block the hosts, tell the command that you want to `work`.

    $ sudo lets work

To unblock the hosts, tell the command that you want to `play`.

    $ sudo lets play

## Disclaimer

`lets` is a toy project to play with Rust. At the time of writing I'm not
considering making it production ready.
