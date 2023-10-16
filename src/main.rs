//
// filter input file by checking its modified time
//
// Usage:
//      # <other_command> | modified_in [diff_in_seconds]
//
// Examples:
//      1) check if Cargo.toml modified in 10 seconds
//      # echo Cargo.toml | modified_in 10
//
//      2) filter result files of locate which is modified in 1 second
//      # locate XXXX | modified_in 1
//

use std::env;
use std::error::Error;
use std::fs;
use std::io;
use std::time::SystemTime;

fn main() {
    let default_diff_in_seconds = String::from("1");

    let args: Vec<String> = env::args().collect();
    let diff_in_seconds = args.get(1).unwrap_or(&default_diff_in_seconds);
    let diff_in_seconds = diff_in_seconds
        .parse::<u64>()
        .expect("diff_in_seconds should be a number");

    let now = now().unwrap();
    let mut line = String::new();

    loop {
        match io::stdin().read_line(&mut line) {
            Ok(len) => {
                if len == 0 {
                    return;
                }

                handle_path(&line, now, diff_in_seconds)
            }
            Err(error) => {
                eprintln!("error: {}", error);
                return;
            }
        }

        line.clear();
    }
}

fn handle_path(line: &String, now: u64, diff_in_seconds: u64) {
    let path = line.trim();

    if path.len() == 0 {
        return;
    }

    let mtime = match mtime(path) {
        Ok(mtime) => mtime,
        Err(_) => return,
    };

    if now - mtime <= diff_in_seconds - 1 {
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
