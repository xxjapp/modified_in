//
// filter input file by checking its modified time
//
// Usage:
//      # <other_command> | modified_in [diff_in_minute]
//
// Examples:
//      1) check if Cargo.toml modified in 10 minutes
//      # echo Cargo.toml | modified_in 10
//
//      2) filter result files of locate which is modified in 1 minutes
//      # locate XXXX | modified_in 1
//

use std::env;
use std::error::Error;
use std::fs;
use std::io;
use std::time::SystemTime;

fn main() {
    let default_diff_in_minute = String::from("1");

    let args: Vec<String> = env::args().collect();
    let diff_in_minute = args.get(1).unwrap_or(&default_diff_in_minute);
    let diff_in_minute = diff_in_minute
        .parse::<u64>()
        .expect("minutes should be number");

    let now = now().unwrap();
    let mut line = String::new();

    loop {
        match io::stdin().read_line(&mut line) {
            Ok(len) => {
                if len == 0 {
                    return;
                }

                handle_path(&line, now, diff_in_minute)
            }
            Err(error) => {
                eprintln!("error: {}", error);
                return;
            }
        }

        line.clear();
    }
}

fn handle_path(line: &String, now: u64, diff_in_minute: u64) {
    let path = line.trim();

    if path.len() == 0 {
        return;
    }

    let mtime = match mtime(path) {
        Ok(mtime) => mtime,
        Err(_) => return,
    };

    if (now - mtime) / 60 <= diff_in_minute - 1 {
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
