# grep-rs

A basic, learning-oriented implementation of `grep`: the famous CLI string and pattern-matching tool.

## How to install & run
- First of all, make sure you have `rust` installed: Go to [rustup.rs](https://rustup.rs/)
- Then execute `cargo build -r` inside the `grep-rs` project root directory.
- Now you can use it from `./target/release/grep-rs`

## What does it do?
`grep-rs` is a CLI tool that searches for a string or a regex pattern in a file, inline text or piped input.

If the string or pattern is found, it will print the line where it was found, with the string or matched pattern colored in red. If the string or pattern is not found, it will print nothing.

## Usage examples
You can type `grep-rs --help` to get a list of all available options.

All examples below uses `grep-rs` as the executable name, but you can use `cargo run --` instead if you are running the code from the repository root directory. For example: `cargo run -- --help`.

### Display help
```bash
grep-rs --help
```

### Search for a string in a file
```bash
grep-rs -f path/to/file "string or regex pattern" 
```

### Search for a string on inline text
```bash
grep-rs -i "inline text" "string or regex pattern" 
```

### Search for a string on a piped input
```bash
echo "piped text" | grep-rs "string or regex pattern" 
```

# License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details

# TODO
- [ ] Add `-d` option to search recursively in a directory
- [ ] Add flag for case-insensitive search
