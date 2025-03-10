# Rust Web Server Logger

A simple command-line tool for continuous logging with log rotation capabilities.

## Building

```bash
cargo build --release
```

## Usage

The program supports three commands:

### Run Command
Starts the server and continuously logs entries every second:

```bash
cargo run -- run
```

The server will continue running until you press Ctrl+C to stop it.

### Count Command
Counts and displays the total number of log entries:

```bash
cargo run -- count
```

This command can be run while the server is running, as it uses file locking for synchronization.

### Rotate Command
Rotates the log files, maintaining up to 5 historical log files:

```bash
cargo run -- rotate
```

This will:
1. Delete http.5.log if it exists
2. Rename http.4.log to http.5.log
3. Rename http.3.log to http.4.log
4. Rename http.2.log to http.3.log
5. Rename http.1.log to http.2.log
6. Rename http.log to http.1.log

## Features

- Continuous logging with 1-second intervals
- Automatic timestamping of log entries
- Log rotation with up to 5 historical log files
- File locking for synchronized access
- Graceful shutdown with Ctrl+C
- Simple command-line interface
- Line counting functionality with synchronized access 