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

---

### Functionality

#### Deleting a File that already has a similar to one that already exists

Current functionality will involve adding a date to the name of the string. This method was chosen as a way to allow people to access older deleted files and not permanently delete or over write the old ones

##### Example

current trash bin --> /Trash/node_modules

folder to delete ---> /isEven/node_modules

The /isEven/node_modules will become: /isEven/node_modules_date (the date, time and seconds deleted)

Will dabble with a `-d` to delete the file if it exists before hand.

---

## Development

- Add a better help message
- Verbose output commandline arg
- Need to add verbose mode
- Need to delete folder after removing files
