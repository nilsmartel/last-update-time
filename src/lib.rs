use std::fs::{read_dir, DirEntry};
use std::io::Result;
use std::time::SystemTime;

pub fn last_update_time(path: &str) -> Result<SystemTime> {
    // smallest thing I would ever need
    let mut smallest_time = SystemTime::UNIX_EPOCH;

    for entry in read_dir(path)? {
        let entry = entry?;

        if let Ok(m) = entry.metadata() {
            let mut time = m
                .modified()
                .unwrap_or_else(|_| m.created().expect("access modified or created time"));

            if m.is_dir() {
                let path = entry.path();
                let path = path.to_str().unwrap();
                let subtime = last_update_time(path)?;
                if subtime < time {
                    time = subtime;
                }
            }

            if time < smallest_time {
                smallest_time = time;
            }
        }
    }

    Ok(smallest_time)
}
