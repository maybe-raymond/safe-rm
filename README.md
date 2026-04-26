# Safe-rm

A Rust-based safety utility for Linux that prevents accidental data loss by moving deleted files to the Trash instead of permanently unlinking them.

Over the weekend I deleted some code by accident with the `rm` command. Spent quite a few hours trying to recover it and ended up re-typing all that code. So I decided to make a safe version in Rust. This project was only written in Rust because I just happened to be learning the language at the time.

**Safe-rm** acts as a buffer, ensuring that your "deleted" files are simply moved to the system Trash folder.

## Compatibility

- **OS:** Linux
- **Tested Environment:** [Omarchy](https://omarchy.org/) (An Arch-based Distro by BaseCamp).
- **Compliance:** Follows standard XDG Trash specifications.

---

## Building & Installation

### 1. Build the Binary

Ensure you have the Rust toolchain installed, then compile the release version:

```bash
cargo build --release
```

### 2. Moving to System Path

To make the command executable from any directory, move the binary to /usr/local/bin/

```bash
sudo mv target/release/safe-rm /usr/local/bin/safe-rm
```

## Usage

The command works just like the normal rm command

```bash
safe-rm a.txt b.txt c.txt d.txt
```

Works with directories

```bash
safe-rm node_modules/
```

Globs also work with the command

```bash
safe-rm *json
```

## Development

- Add a better help message
- Verbose output commandline arg
- Need to add verbose mode
- Need to delete folder after removing files
