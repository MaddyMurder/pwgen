# PWGEN

A simple CLI password generator written in Rust.

## Features

Generate strong passwords at any length with customizable character sets. Automatically copy to clipboard. And generate simple usernames.

## Installation

Install by going to the releases section and downloading the appropriate binary for your system, then add it to the path to easily access it with you command-line interface.

## Documentation

Documentation is built-in. Access it by using `pwgen help`. Learn more about the subcommands using `pwgen help [subcommand]`.

## Example Usage

Generate a password with all available character sets, do not hide it, and make it 32 characters long:
```bash
pwgen password -c lower,upper,digits,symbol,rare-symbol -i -l 32
```
**Output:**
```
,v|v}1/g>)JRklF;!GdgqEU)%uhTP>\|
Password copied to clipboard.
```

Generate a password that's 8 characters long, only includes the numbers `0` and `1`, do not copy to clipboard and do not hide it:
```bash
pwgen password --length 8 --char-set digits --exclude 23456789 --no-copy --no-hide
```
**Output:**
```
00110100
```

Generate a username with a dash in between the sections and 4 numbers at the end:
```bash
pwgen username -s "-" --numbers 4
```
**Output:**
```
irritating-spud-4295
Username copied to clipboard.
```

## Building from Source

To build PWGEN from source, install cargo and execute `cargo build`.

## License

PWGEN is licenced under GNU AGPLv3. The full license can be found in the `LICENSE` file