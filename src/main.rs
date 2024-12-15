use std::env;
use std::error::Error;
use std::fs;
use std::io;
use std::time::SystemTime;

const VERSION: &str = "1.0.0";

fn main() {
    let args: Vec<String> = env::args().collect();

    // Handle --help and --version
    if args.len() > 1 {
        match args[1].as_str() {
            "--help" => {
                print_help();
                return;
            }
            "--version" => {
                println!("modified_in version {}", VERSION);
                return;
            }
            _ => {}
        }
    }

    let default_diff_in_seconds = String::from("1");
    let diff_in_seconds = args.get(1).unwrap_or(&default_diff_in_seconds);
    let diff_in_seconds = match diff_in_seconds.parse::<u64>() {
        Ok(seconds) => seconds,
        Err(_) => {
            eprintln!("Error: diff_in_seconds should be a positive integer.");
            return;
        }
    };

    let now = now().unwrap_or_else(|e| {
        eprintln!("Failed to get current time: {}", e);
        std::process::exit(1);
    });

    // Pre-calculate the threshold
    let min_allowed_mtime = now - diff_in_seconds + 1;

    let mut line = String::new();

    loop {
        match io::stdin().read_line(&mut line) {
            Ok(len) => {
                if len == 0 {
                    return;
                }

                handle_path(&line, min_allowed_mtime);
            }
            Err(error) => {
                eprintln!("Error reading input: {}", error);
                return;
            }
        }

        line.clear();
    }
}

fn handle_path(line: &str, min_allowed_mtime: u64) {
    let path = line.trim();

    if path.is_empty() {
        return;
    }

    let mtime = match mtime(path) {
        Ok(mtime) => mtime,
        Err(_) => return,
    };

    if mtime >= min_allowed_mtime {
        println!("{}", path);
    }
}

// get file/folder modified time in seconds
fn mtime(path: &str) -> Result<u64, Box<dyn Error>> {
    since_epoch(&fs::metadata(path)?.modified()?)
}

// get now time in seconds
fn now() -> Result<u64, Box<dyn Error>> {
    since_epoch(&SystemTime::now())
}

// get time value in seconds since epoch
fn since_epoch(t: &SystemTime) -> Result<u64, Box<dyn Error>> {
    Ok(t.duration_since(SystemTime::UNIX_EPOCH)?.as_secs())
}

fn print_help() {
    println!(
        r#"
modified_in: Filter input files by their modified time.

This tool filters a list of file paths provided via stdin, keeping only those
files that have been modified within the specified time interval.

Usage:
  <command> | modified_in [diff_in_seconds]
  modified_in --help
  modified_in --version

Options:
  --help       Show this help message.
  --version    Show version information.

Examples:
  1) Check if Cargo.toml has been modified within 10 seconds:
     echo Cargo.toml | modified_in 10

  2) Filter files modified in 1 second from locate results:
     locate XXXX | modified_in 1
"#
    );
}
