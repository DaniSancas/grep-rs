# grep-rs

A basic, learning-oriented implementation of `grep`: the famous CLI string and pattern-matching tool.

## How to install & run
- First of all, make sure you have `rust` installed: Go to [rustup.rs](https://rustup.rs/)
- Then execute `cargo build -r` inside the `grep-rs` project root directory.
- Now you can use it from `./target/release/grep-rs`

## Usage examples
You can type `grep-rs --help` to get a list of all available options.

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

# TODO
- [ ] Add examples to the docs
- [ ] Add `-d` option to search recursively in a directory
- [ ] Add flag for case-insensitive search
- [ ] Allow to pipe input from `stdin`

# License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details
