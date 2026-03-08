# Safe-rm

A Rust-based safety utility for Linux that prevents accidental data loss by moving deleted files to the Trash instead of permanently unlinking them.

## The Backstory

We've all been there: a mistyped `rm` command deletes hours of work in a heartbeat. After losing code to the standard Linux `rm` command and finding no way to recover it, I saw an opportunity to build a "safe" alternative.

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

## Development

- [] Add wildcard for files to do something like the following `safe-rm *.txt *.json`
- [] Add a better help message
- [] Verbose output commandline arg
