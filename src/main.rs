//
// filter input file by checking its modified time
//
// Usage examples:
//      1) check if Cargo.toml modified in 10 minutes
//      # echo Cargo.toml | modified_in 10
//
//      2) filter result files of locate which is modified in 1 minutes
//      # locate XXXX | modified_in 1
//

use filetime::FileTime;
use std::env;
use std::fs;
use std::io;
use std::time::SystemTime;

fn main() {
    let args: Vec<String> = env::args().collect();
    let diff_in_minute = args.get(1).expect("minutes not found");
    let diff_in_minute = diff_in_minute
        .parse::<i64>()
        .expect("minutes should be number");

    let now = now_as_secs();
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

fn handle_path(line: &String, now: u64, diff_in_minute: i64) {
    let path = line.trim();

    if path.len() == 0 {
        return;
    }

    let mtime = mtime_as_secs(path);

    if (now as i64 - mtime) / 60 <= diff_in_minute - 1 {
        println!("{}", path);
    }
}

fn mtime_as_secs(path: &str) -> i64 {
    match fs::metadata(path) {
        Ok(metadata) => {
            let mtime = FileTime::from_last_modification_time(&metadata);
            mtime.unix_seconds()
        }
        Err(_) => 0,
    }
}

fn now_as_secs() -> u64 {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("SystemTime before UNIX EPOCH!");
    now.as_secs()
}
